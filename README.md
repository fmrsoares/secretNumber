# Secret Number

This is a simple secret Contract in Rust to run in [Secret Network](https://github.com/enigmampc/SecretNetwork).

## Description 
Secret Number enables anyone to store a secret number on the secret network and other people try to guess it.
    - Inst - Called when instanciated
        - secret_number: Secret Number that has to be found
        - owner: Instanciator address
    - GuessNumber - Can be called by anyone
        - guess_number: Number that is compared to the secretNumber and returns
            - secret_number == guess_number - Congratulations, the secret number is ${guess_number}
            - secret_number > guess_number - The secret number is higher than ${guess_number}
            - secret_number < guess_number - The secret number is lower than ${guess_number}
    - ModifySecretNumber - Can only be called by the instanciator
        - new_secret_number: New Secret Number


startContractScript.sh script runs the cosmwasm optimizer on the source code and then initializes a test local secret network using docker
testContractScript.sh stores the contract on the blockchain, instanciates it with a secret_number and then it calls the contract functions
