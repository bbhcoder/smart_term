pub fn generate() -> String {
    String::from(
        "$global:SmartTermOriginalPrompt = $function:prompt\n\
        function global:prompt {\n\
            $exit_code = $LASTEXITCODE\n\
            smart hook pre-cmd $exit_code | Out-Null\n\
            $sp = smart prompt\n\
            if (![string]::IsNullOrEmpty($sp)) {\n\
                Write-Host -NoNewline -ForegroundColor Yellow $sp\n\
            }\n\
            & $global:SmartTermOriginalPrompt\n\
        }\n\
        if (Get-Module -ListAvailable PSReadLine) {\n\
            Set-PSReadLineKeyHandler -Key Enter -ScriptBlock {\n\
                $line = $null; $cursor = $null\n\
                [Microsoft.PowerShell.PSConsoleReadLine]::GetBufferState([ref]$line, [ref]$cursor)\n\
                if (-not [string]::IsNullOrWhiteSpace($line)) {\n\
                    smart hook pre-exec $line | Out-Null\n\
                }\n\
                [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()\n\
            }\n\
        }\n"
    )
}
