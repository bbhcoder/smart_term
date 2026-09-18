Write-Host "[1/4] Detecting OS and native downloaders..." -ForegroundColor Green
$Url = "https://github.com/bbhcoder/smart_term/releases/latest/download/smart_term-windows.msi"
$OutFile = "$env:TEMP\smart_term.msi"

Write-Host "[2/4] Downloading SmartTerm for Windows..." -ForegroundColor Green
Invoke-WebRequest -Uri $Url -OutFile $OutFile

Write-Host "[3/4] Installing..." -ForegroundColor Green
Start-Process -FilePath "msiexec.exe" -ArgumentList "/i $OutFile /qn" -Wait -NoNewWindow

Write-Host "[4/4] Configuration" -ForegroundColor Green
$response = Read-Host "Do you want SmartTerm to launch automatically in new PowerShell sessions? (y/n)"
if ($response -match "^[yY]") {
    if (Get-Command smart -ErrorAction SilentlyContinue) {
        smart org
    }
}

Write-Host "[Success] Installation complete! Open a new terminal to see it in action." -ForegroundColor Green