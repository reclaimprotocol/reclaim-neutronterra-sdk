import { getAccountByName } from "@kubiklabs/wasmkit";
import { LCDClient, MnemonicKey } from '@terra-money/terra.js';
import { Proof, ClaimInfo, CompleteClaimData, SignedClaim, ProofMsg, NeutronIbcContract } from "../artifacts/typescript_schema/NeutronIbcContract";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const neutronIbcContract = new NeutronIbcContract();
  await neutronIbcContract.setupClient();

  const owner = "0xe4c20c9f558160ec08106de300326f7e9c73fb7f"

  const claimInfo: ClaimInfo = {
      provider: "http",
      parameters: "{\"body\":\"\",\"geoLocation\":\"in\",\"method\":\"GET\",\"responseMatches\":[{\"type\":\"contains\",\"value\":\"_steamid\\\">Steam ID: 76561199632643233</div>\"}],\"responseRedactions\":[{\"jsonPath\":\"\",\"regex\":\"_steamid\\\">Steam ID: (.*)</div>\",\"xPath\":\"id(\\\"responsive_page_template_content\\\")/div[@class=\\\"page_header_ctn\\\"]/div[@class=\\\"page_content\\\"]/div[@class=\\\"youraccount_steamid\\\"]\"}],\"url\":\"https://store.steampowered.com/account/\"}",
      context: "{\"contextAddress\":\"user's address\",\"contextMessage\":\"for acmecorp.com on 1st january\"}",
  }

  const identifier = "0x531322a6c34e5a71296a5ee07af13f0c27b5b1e50616f816374aff6064daaf55"

  const completeClaimData:CompleteClaimData = {
        identifier: identifier,
        owner: owner,
        epoch: 1,
        timestampS: 1710157447
  }
  const signedClaim: SignedClaim = {
      claim: completeClaimData,
      signatures: ["0x52e2a591f51351c1883559f8b6c6264b9cb5984d0b7ccc805078571242166b357994460a1bf8f9903c4130f67d358d7d6e9a52df9a38c51db6a10574b946884c1b"],
  }

  const proof: Proof = {
      claimInfo: claimInfo,
      signedClaim: signedClaim
  }

  const proofMsg: ProofMsg = {
    proof: proof
  }
  const ex_response = await neutronIbcContract.verifyProof(
    {
      account: contract_owner,
    },
    {
      channel: "channel-4282",
      proof: proofMsg
    }
  );
  console.log(ex_response);

  // Terra Part
  const terra = new LCDClient({
    URL: "https://phoenix-lcd.terra.dev", // Use "https://lcd.terra.dev" for prod "http://localhost:1317" for localterra.
    chainID: "phoenix-1", // Use "columbus-5" for production or "localterra".
  });

  const queryResult = await terra.wasm.contractQuery(
    "terra13z58nvwxf5hjx0l7pwd72tt4dd2ycvj3zp3n5m5vtr3wnfw2jdzss35maw",
    { query_result: { identifier: identifier} } // query msg
  );
  console.log(queryResult); 
}
