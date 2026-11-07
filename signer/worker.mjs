// The Arkyv Pro signer (Cloudflare Worker). Arkyv posts a Lemon Squeezy
// license key; if Lemon Squeezy says it's an active key for Arkyv Pro and the
// key has a free activation, the Worker answers with a key signed with
// Arkyv's private key, which the app then checks offline. Nothing is stored
// here: Lemon Squeezy keeps the activations.
//
//   POST /            {license_key, instance_id?, instance_name}
//                     → 200 {key, instance_id} · 404 not ours · 409 limit reached
//   POST /deactivate  {license_key, instance_id} → 200 · 404
//
// A computer that already has an activation sends its instance_id, and is
// signed again without using another one.
//
// Secrets and variables (see README.md):
//   LICENSE_PRIVATE_KEY  secret: ~/crate-license/license-ed25519.pem (PKCS#8 PEM)
//   LS_STORE_ID          the Lemon Squeezy store's id
//   LS_PRODUCT_ID        Arkyv Pro's product id
//   MAX_ACTIVATIONS      computers per key (default 5), checked here as well,
//                        so a missing limit in Lemon Squeezy doesn't open it up

import { payload, encode } from "./key.mjs";

const json = (status, body) =>
  new Response(JSON.stringify(body), { status, headers: { "content-type": "application/json" } });

async function privateKey(pem) {
  const b64 = pem.replace(/-----[^-]+-----/g, "").replace(/\s+/g, "");
  const der = Uint8Array.from(atob(b64), (c) => c.charCodeAt(0));
  return crypto.subtle.importKey("pkcs8", der, { name: "Ed25519" }, false, ["sign"]);
}

// Lemon Squeezy's License API: form-encoded, no API key needed.
async function lemon(action, fields) {
  const res = await fetch(`https://api.lemonsqueezy.com/v1/licenses/${action}`, {
    method: "POST",
    headers: { accept: "application/json", "content-type": "application/x-www-form-urlencoded" },
    body: new URLSearchParams(fields),
  });
  if (res.status >= 500) throw new Error("Lemon Squeezy is unavailable");
  return res.json().catch(() => ({}));
}

const ours = (answer, env) =>
  String(answer.meta?.store_id) === String(env.LS_STORE_ID) &&
  String(answer.meta?.product_id) === String(env.LS_PRODUCT_ID) &&
  answer.license_key?.status !== "disabled";

async function signed(answer, env) {
  const bytes = payload(`ls:${answer.meta.order_id}`);
  const signature = await crypto.subtle.sign("Ed25519", await privateKey(env.LICENSE_PRIVATE_KEY), bytes);
  return encode(bytes, signature);
}

async function activate(body, env) {
  const licenseKey = body.license_key;
  const max = Number(env.MAX_ACTIVATIONS || 5);

  // This computer is already activated: check that activation, use no other.
  if (body.instance_id) {
    const answer = await lemon("validate", { license_key: licenseKey, instance_id: String(body.instance_id) });
    if (answer.valid && answer.instance && ours(answer, env)) {
      return json(200, { key: await signed(answer, env), instance_id: answer.instance.id });
    }
    // Deactivated meanwhile (from another computer, or by support): activate anew.
  }

  const name = String(body.instance_name || "Arkyv").slice(0, 100);
  const answer = await lemon("activate", { license_key: licenseKey, instance_name: name });
  if (!answer.activated) {
    const limit = /limit/i.test(String(answer.error || ""));
    return limit ? json(409, { error: "limit" }) : json(404, { error: "not a Arkyv Pro key" });
  }
  if (!ours(answer, env)) {
    await lemon("deactivate", { license_key: licenseKey, instance_id: answer.instance.id });
    return json(404, { error: "not a Arkyv Pro key" });
  }
  if (Number(answer.license_key?.activation_usage) > max) {
    await lemon("deactivate", { license_key: licenseKey, instance_id: answer.instance.id });
    return json(409, { error: "limit" });
  }
  return json(200, { key: await signed(answer, env), instance_id: answer.instance.id });
}

async function deactivate(body) {
  if (!body.instance_id) return json(400, { error: "no instance_id" });
  const answer = await lemon("deactivate", { license_key: body.license_key, instance_id: String(body.instance_id) });
  return answer.deactivated ? json(200, {}) : json(404, { error: "no such activation" });
}

export default {
  async fetch(request, env) {
    if (request.method !== "POST") return json(405, { error: "POST only" });
    let body;
    try {
      body = await request.json();
    } catch {
      return json(400, { error: "expected JSON" });
    }
    body.license_key = String(body.license_key ?? "").trim();
    if (!body.license_key || body.license_key.length > 200) return json(400, { error: "no license key" });
    try {
      const path = new URL(request.url).pathname.replace(/\/+$/, "");
      return path === "/deactivate" ? await deactivate(body) : await activate(body, env);
    } catch (e) {
      return json(502, { error: String(e.message || e) });
    }
  },
};
