Write-Host "[1/3] Uninstalling SmartTerm..." -ForegroundColor Yellow
$app = Get-WmiObject -Class Win32_Product \vert{} Where-Object {$_.Name -match "smart_term" }
if ($app) {$app.Uninstall() | Out-Null }Write-Host "[2/3] Cleaning up PowerShell profile..." -ForegroundColor Yellow
$profilePath =$PROFILE
if (Test-Path $profilePath) {
$content = Get-Content$profilePath
$newContent =$content | Where-Object { $_ -notmatch "SMART_TERM_ACTIVE" -and $_ -notmatch "SmartTerm" }
Set-Content -Path $profilePath -Value$newContent
}$response = Read-Host "[3/3] Delete history databases? (y/n)"
if ($response -match "^[yY]") {
$dbPath = Join-Path$env:USERPROFILE ".smart_term_history.sqlite"
$devDbPath = Join-Path$env:USERPROFILE ".smart_term_dev_history.sqlite"
if (Test-Path $dbPath) { Remove-Item -Path$dbPath -Force }
if (Test-Path $devDbPath) { Remove-Item -Path$devDbPath -Force }
}Write-Host "[Success] SmartTerm has been completely uninstalled." -ForegroundColor Green