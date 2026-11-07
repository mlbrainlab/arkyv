; Inno Setup script for Arkyv on Windows (Inno Setup 6). Built by
; scripts/package-windows.ps1, which passes the version and the folder with
; the release binaries:
;
;   iscc /DVersion=0.2.0 /DBin=..\..\target\release packaging\windows\arkyv.iss
;
; Installs for the current user by default (no administrator rights needed);
; "Install for all users" is offered in the first page, or /ALLUSERS on the
; command line. Registry keys go under HKCU or HKLM to match (HKA).

#ifndef Version
  #error Pass /DVersion=x.y.z
#endif
#ifndef Bin
  #define Bin "..\..\target\release"
#endif

; The ProgID archives point at, and the name in Settings → Default apps.
#define ProgId "Arkyv.Archive"
#define AppKey "Software\Arkyv"
; The app's name before 0.2.36. An update over it removes its programs,
; shortcut and registry entries, so Explorer doesn't offer both.
#define OldName "Crate"

[Setup]
AppId={{6C1C2A4E-6E9B-4E39-9F0B-3A0F6D3C2B71}
AppName=Arkyv
AppVersion={#Version}
AppVerName=Arkyv {#Version}
AppPublisher=mlbrainlab
AppPublisherURL=https://github.com/mlbrainlab/arkyv
AppSupportURL=https://github.com/mlbrainlab/arkyv/issues
AppUpdatesURL=https://github.com/mlbrainlab/arkyv/releases
DefaultDirName={autopf}\Arkyv
DefaultGroupName=Arkyv
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=dialog commandline
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
OutputDir=..\..\target\dist
OutputBaseFilename=Arkyv-{#Version}-x64-setup
SetupIconFile=..\..\assets\icons\arkyv.ico
UninstallDisplayIcon={app}\arkyv-app.exe
UninstallDisplayName=Arkyv
LicenseFile=..\..\LICENSE
WizardStyle=modern
Compression=lzma2/max
SolidCompression=yes
ChangesAssociations=yes
CloseApplications=yes

[Files]
Source: "{#Bin}\arkyv-app.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#Bin}\arkyv.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"; Flags: ignoreversion

[InstallDelete]
Type: files; Name: "{app}\{#LowerCase(OldName)}-app.exe"
Type: files; Name: "{app}\{#LowerCase(OldName)}.exe"
Type: files; Name: "{autoprograms}\{#OldName}.lnk"

[Icons]
Name: "{autoprograms}\Arkyv"; Filename: "{app}\arkyv-app.exe"; Comment: "Open, extract and create archives"

[Registry]
Root: HKA; Subkey: "Software\Classes\{#OldName}.Archive"; ValueType: none; Flags: deletekey
Root: HKA; Subkey: "Software\Classes\Applications\{#LowerCase(OldName)}-app.exe"; ValueType: none; Flags: deletekey
Root: HKA; Subkey: "Software\{#OldName}"; ValueType: none; Flags: deletekey
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: none; ValueName: "{#OldName}"; Flags: deletevalue
Root: HKA; Subkey: "Software\Classes\*\shell\{#OldName}.Compress"; ValueType: none; Flags: deletekey
Root: HKA; Subkey: "Software\Classes\Directory\shell\{#OldName}.Compress"; ValueType: none; Flags: deletekey
; The archive type: icon and what double-click does once Arkyv is the default.
Root: HKA; Subkey: "Software\Classes\{#ProgId}"; ValueType: string; ValueName: ""; ValueData: "Archive"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\{#ProgId}"; ValueType: string; ValueName: "FriendlyTypeName"; ValueData: "Archive"
Root: HKA; Subkey: "Software\Classes\{#ProgId}\DefaultIcon"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"",0"
Root: HKA; Subkey: "Software\Classes\{#ProgId}\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"" ""%1"""

; "Open with" for the app itself.
Root: HKA; Subkey: "Software\Classes\Applications\arkyv-app.exe"; ValueType: string; ValueName: "FriendlyAppName"; ValueData: "Arkyv"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Applications\arkyv-app.exe\shell\open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"" ""%1"""

; Settings → Default apps lists Arkyv under this name; the first run opens
; that page (ms-settings:defaultapps?registeredAppUser=Arkyv).
Root: HKA; Subkey: "{#AppKey}"; Flags: uninsdeletekey
Root: HKA; Subkey: "{#AppKey}\Capabilities"; ValueType: string; ValueName: "ApplicationName"; ValueData: "Arkyv"
Root: HKA; Subkey: "{#AppKey}\Capabilities"; ValueType: string; ValueName: "ApplicationDescription"; ValueData: "Open, extract and create archives"
Root: HKA; Subkey: "{#AppKey}\Capabilities"; ValueType: string; ValueName: "ApplicationIcon"; ValueData: """{app}\arkyv-app.exe"",0"
Root: HKA; Subkey: "Software\RegisteredApplications"; ValueType: string; ValueName: "Arkyv"; ValueData: "{#AppKey}\Capabilities"; Flags: uninsdeletevalue

; For each archive extension: listed under "Open with", in Default apps, and
; Explorer's right-click entries (classic verbs; on Windows 11 they're under
; "Show more options").
#dim Exts[13] {".zip", ".7z", ".rar", ".tar", ".tgz", ".gz", ".txz", ".xz", ".tzst", ".zst", ".tbz2", ".bz2", ".001"}
#define I
#sub RegisterExt
Root: HKA; Subkey: "Software\Classes\{#Exts[I]}\OpenWithProgids"; ValueType: string; ValueName: "{#ProgId}"; ValueData: ""; Flags: uninsdeletevalue
Root: HKA; Subkey: "Software\Classes\Applications\arkyv-app.exe\SupportedTypes"; ValueType: string; ValueName: "{#Exts[I]}"; ValueData: ""
Root: HKA; Subkey: "{#AppKey}\Capabilities\FileAssociations"; ValueType: string; ValueName: "{#Exts[I]}"; ValueData: "{#ProgId}"
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\Arkyv.Extract"; ValueType: string; ValueName: ""; ValueData: "Extract with Arkyv"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\Arkyv.Extract"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\arkyv-app.exe"",0"
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\Arkyv.Extract\command"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"" --extract-each ""%1"""
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\Arkyv.Open"; ValueType: string; ValueName: ""; ValueData: "Open with Arkyv"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\Arkyv.Open"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\arkyv-app.exe"",0"
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\Arkyv.Open\command"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"" ""%1"""
Root: HKA; Subkey: "Software\Classes\{#Exts[I]}\OpenWithProgids"; ValueType: none; ValueName: "{#OldName}.Archive"; Flags: deletevalue
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\{#OldName}.Extract"; ValueType: none; Flags: deletekey
Root: HKA; Subkey: "Software\Classes\SystemFileAssociations\{#Exts[I]}\shell\{#OldName}.Open"; ValueType: none; Flags: deletekey
#endsub
#for {I = 0; I < DimOf(Exts); I++} RegisterExt

; Any file or folder: "Compress with Arkyv…" (the New archive dialog).
Root: HKA; Subkey: "Software\Classes\*\shell\Arkyv.Compress"; ValueType: string; ValueName: ""; ValueData: "Compress with Arkyv…"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\*\shell\Arkyv.Compress"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\arkyv-app.exe"",0"
Root: HKA; Subkey: "Software\Classes\*\shell\Arkyv.Compress\command"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"" --compress ""%1"""
Root: HKA; Subkey: "Software\Classes\Directory\shell\Arkyv.Compress"; ValueType: string; ValueName: ""; ValueData: "Compress with Arkyv…"; Flags: uninsdeletekey
Root: HKA; Subkey: "Software\Classes\Directory\shell\Arkyv.Compress"; ValueType: string; ValueName: "Icon"; ValueData: """{app}\arkyv-app.exe"",0"
Root: HKA; Subkey: "Software\Classes\Directory\shell\Arkyv.Compress\command"; ValueType: string; ValueName: ""; ValueData: """{app}\arkyv-app.exe"" --compress ""%1"""

[Run]
Filename: "{app}\arkyv-app.exe"; Description: "Open Arkyv"; Flags: nowait postinstall skipifsilent
