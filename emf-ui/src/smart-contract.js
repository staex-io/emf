import { ContractPromise } from '@polkadot/api-contract'
import { ApiPromise, WsProvider } from '@polkadot/api'
import metadata from '@/assets/emf_contract.metadata.json'

let globalApi = null
let globalContract = null

export const initializeApiContract = async () => {
  // Return already initialized connection.
  if (globalApi !== null && globalContract !== null) {
    return { api: globalApi, contract: globalContract }
  }

  // Connect to Substrate and init API and contract.
  const provider = new WsProvider('ws://127.0.0.1:9944')
  const api = await ApiPromise.create({ provider })
  const [chain, nodeName, nodeVersion] = await Promise.all([
    api.rpc.system.chain(),
    api.rpc.system.name(),
    api.rpc.system.version(),
  ])
  console.log(`You are connected to chain ${chain} using ${nodeName} v${nodeVersion}`)
  const contract = new ContractPromise(api, metadata, import.meta.env.VITE_CONTRACT_ADDRESS)

  globalApi = api
  globalContract = contract
  return { api: globalApi, contract: globalContract }
}
