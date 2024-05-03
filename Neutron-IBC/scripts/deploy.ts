import { getAccountByName } from "@kubiklabs/wasmkit";

import { NeutronIbcContract } from "../artifacts/typescript_schema/NeutronIbcContract";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const neutronIbcContract = new NeutronIbcContract();
  await neutronIbcContract.setupClient();

  const deploy_response = await neutronIbcContract.deploy(
    contract_owner,
  );
  console.log(deploy_response);

  const contract_info = await neutronIbcContract.instantiate(
    {},
    `deploy test ${runTs}`,
    contract_owner,
    undefined,  // transferAmount
    // customFees, // custom fees here
  );
  console.log(contract_info);
}
