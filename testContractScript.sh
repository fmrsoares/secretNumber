docker exec -it secretdev secretcli tx compute store code/contract.wasm --from a --gas 100000000 -y --keyring-backend test
sleep 5
docker exec -it secretdev secretcli tx compute instantiate 1 "{\"secret_number\": 10}" --from a --label "secretNumber" -y --keyring-backend test
sleep 5
docker exec -it secretdev secretcli query compute query secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg "{\"guess_number\": {\"guess_number\": 5}}"
sleep 1
docker exec -it secretdev secretcli query compute query secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg "{\"guess_number\": {\"guess_number\": 15}}"
sleep 1
docker exec -it secretdev secretcli query compute query secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg "{\"guess_number\": {\"guess_number\": 10}}"
sleep 1
docker exec -it secretdev secretcli tx compute execute secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg "{\"modify_secret_number\": {\"new_secret_number\": 5}}" --from a --keyring-backend test
sleep 5
docker exec -it secretdev secretcli query compute query secret18vd8fpwxzck93qlwghaj6arh4p7c5n8978vsyg "{\"guess_number\": {\"guess_number\": 5}}"