Write-Host "[1/4] Detecting OS and native downloaders..." -ForegroundColor Green
$Url = "https://github.com/bbhcoder/smart_term/releases/latest/download/smart_term-windows.msi"
$OutFile = "$env:TEMP\smart_term.msi"

Write-Host "[2/4] Downloading SmartTerm for Windows..." -ForegroundColor Green
Invoke-WebRequest -Uri $Url -OutFile$OutFile

Write-Host "[3/4] Installing..." -ForegroundColor Green
Start-Process -FilePath "msiexec.exe" -ArgumentList "/i $OutFile /qn" -Wait -NoNewWindow

Write-Host "[4/4] Configuration" -ForegroundColor Green
$response = Read-Host "Do you want SmartTerm to launch automatically in new PowerShell sessions? (y/n)"
if ($response -match "^[yY]") {
    $profilePath =$PROFILE
    if (-not (Test-Path $profilePath)) {
        New-Item -Path (Split-Path $profilePath) -ItemType Directory -Force | Out-Null
        New-Item -Path $profilePath -ItemType File -Force | Out-Null
    }
    
    $profileContent = Get-Content$profilePath -ErrorAction SilentlyContinue
    if ($profileContent -notmatch "smart") {
        Add-Content -Path $profilePath -Value "`n# Launch SmartTerm`nif (Get-Command smart -ErrorAction SilentlyContinue) { smart }"
    }
    Write-Host "[System] SmartTerm set as default environment!" -ForegroundColor Green
}

Write-Host "[Success] Installation complete! Open a new terminal to see it in action." -ForegroundColor Green