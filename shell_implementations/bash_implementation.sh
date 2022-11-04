#!/bin/bash

# To make this script usable just copy the function below
# and paste it on your ~/.bashrc

goto() {
    EXCHANGE_PATH="/tmp/gotochoice"
    ## The first one is useful for testing, the second one is the one 
    ## supposed to be used in production.
    #GOTO="target/debug/goto"
    GOTO=$(which goto)

    # Make sure an old path its not used.
    if [ -f $EXCHANGE_PATH ]; then 
        rm $EXCHANGE_PATH
    fi

    if [ -f $GOTO ]; then
        $GOTO $@


        if [ -f $EXCHANGE_PATH ]; then 
            TARGET_PATH=$(cat $EXCHANGE_PATH)
            echo "Setting location to '$TARGET_PATH'..."
            cd $TARGET_PATH
            rm $EXCHANGE_PATH
        else
            # Nothing was selected in the goto process
            echo "Nothing was selected in goto."
        fi
    else
        echo "The goto path executable doesn't exist."
    fi
}

goto $@
