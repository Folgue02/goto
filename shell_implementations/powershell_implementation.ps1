#!/opt/microsoft/powershell/7/pwsh

# To make this script usable just copy the function below and paste
# it in your $profile file (`~/.config/powershell/Microsoft.Powershell_profile.ps1`)

function goto ([String]$whereto=$null) {
    ## The first one is useful for testing, the second one is the one 
    ## supposed to be used in production

    #$goto = (Join-Path "target" "debug" "goto")
    $goto = which "goto"

    $exchange_path = Join-Path "/" "tmp" "gotochoice"


    if (Test-Path $goto) {

        # Make sure an old path its not used.
        if (Test-Path $exchange_path) {
            Remove-Item $exchange_path
        }

        if ($whereto -eq $null) {
            Start-Process $goto -Wait
        } else {
            Start-Process $goto -Wait -Args $whereto
        }

        if (Test-Path $exchange_path) {
            $target_path = (Get-Content $exchange_path).Trim()
            Write-Host "Setting location to '$target_path'..."
            Set-Location $target_path
            Remove-Item $exchange_path
        } else {
            # Nothing was selected in the goto process.
            Write-Host "Nothing was selected in goto."
        }
    } else {
        Write-Host "The goto path executable doesn't exist." -ForegroundColor Red    
    }
}

goto $args
