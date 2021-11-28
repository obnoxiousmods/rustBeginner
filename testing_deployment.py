from terra_sdk.client.lcd import LCDClient

terra = LCDClient(chain_id="columbus-4", url="https://lcd.terra.dev")
print(terra.tendermint.node_info())
