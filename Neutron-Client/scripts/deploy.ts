import { getAccountByName } from "@kubiklabs/wasmkit";

import { NeutronClientContract } from "../artifacts/typescript_schema/NeutronClientContract";

export default async function run () {
  const runTs = String(new Date());
  const contract_owner = await getAccountByName("account_0");
  const neutronClientContract = new NeutronClientContract();
  await neutronClientContract.setupClient();

  const deploy_response = await neutronClientContract.deploy(
    contract_owner,
  );
  console.log(deploy_response);

  const contract_info = await neutronClientContract.instantiate(
    {},
    `deploy test ${runTs}`,
    contract_owner,
    undefined,  // transferAmount
    // customFees, // custom fees here
  );
  console.log(contract_info);
}
