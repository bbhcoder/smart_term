Write-Host "[1/3] Detecting OS and native downloaders..." -ForegroundColor Green
$Url = "https://github.com/bbhcoder/smart_term/releases/latest/download/smart_term-windows.msi"
$OutFile = "$env:TEMP\smart_term.msi"

Write-Host "[2/3] Downloading SmartTerm for Windows..." -ForegroundColor Green
Invoke-WebRequest -Uri $Url -OutFile $OutFile

Write-Host "[3/3] Installing..." -ForegroundColor Green
Start-Process -FilePath "msiexec.exe" -ArgumentList "/i $OutFile /qn" -Wait -NoNewWindow

Write-Host "[Success] SmartTerm installed! Open a new terminal and type 'smart'." -ForegroundColor Green