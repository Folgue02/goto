#!/opt/microsoft/powershell/7/pwsh

# To make this script usable just copy this code and wrap
# it around a function inside of your $profile
# Example: 
# ``` powershell
# # Your $profile file.
# function Goto-Place {
#     # Here goes the code   
# }
# ```

function goto () {
    ## The first one is useful for testing, the second one is the one 
    ## supposed to be used in production
    $goto = (Join-Path "target" "debug" "goto")
    #$goto = which "goto"

    $exchange_path = Join-Path "/" "tmp" "gotochoice"


    if (Test-Path $goto) {

        # Make sure an old path its not used.
        if (Test-Path $exchange_path) {
            Remove-Item $exchange_path
        }

        Start-Process $goto -Wait
        Write-Host $exchange_path -ForegroundColor Yellow
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

goto