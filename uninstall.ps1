Stop-Process -Name "smartd" -Force -ErrorAction SilentlyContinue
Stop-Process -Name "smart" -Force -ErrorAction SilentlyContinue
$InstallDir = "$env:LOCALAPPDATA\SmartTerm"
if (Test-Path $InstallDir) {
    Remove-Item -Path $InstallDir -Recurse -Force
}
$UserPath = [Environment]::GetEnvironmentVariable("PATH", "User")
$CleanPath = ($UserPath -split ';' | Where-Object { $_ -ne $InstallDir -and $_ -ne "" }) -join ';'
[Environment]::SetEnvironmentVariable("PATH", $CleanPath, "User")
$env:PATH = ($env:PATH -split ';' | Where-Object { $_ -ne $InstallDir -and $_ -ne "" }) -join ';'
$ProfilePath = $PROFILE
if (Test-Path $ProfilePath) {
    $ProfileContent = Get-Content $ProfilePath -Raw
    $CleanProfile = $ProfileContent -replace '(?m)^.*smart init powershell.*$\r?\n?', ''
    Set-Content -Path $ProfilePath -Value $CleanProfile -Force
}
Write-Host "SmartTerm removed. Database kept safely at ~/.smart_dev/smart_term.db" -ForegroundColor Green
