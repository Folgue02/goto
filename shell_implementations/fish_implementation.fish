#!/usr/bin/fish


# To make this script usable just copy this code and wrap
# it around a function inside of your ~/.config/fish/config.fish


function goto
    ## The first one is useful for testing, the second one is the one 
    ## supposed to be used in production
    #set goto "target/debug/goto"
    set goto (which goto)

    set exchange_path "/tmp/gotochoice"

    if test -f $goto

        # Make sure an old path its not used
        if test -f $exchange_path
            rm $exchange_path
        end

        $goto

        if test -f $exchange_path
            set target_path (/bin/cat $exchange_path)
            cd $target_path
            rm $exchange_path
        else
            echo "Nothing was selected in goto."
        end
    else 
        echo "The goto path executable doesn't exist."
    end
end

goto