import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { TxButton } from './substrate-lib/components'
import KittyCards from './KittyCards'
import { useSubstrateState } from './substrate-lib'
const acctAddr = acct => (acct ? acct.address : '')

export default function Main (props) {
  const { api, currentAccount } = useSubstrateState()

  // const [kittyCnt] = useState(0)
  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  function toByteArray(hexString) {
    if (hexString.substr(0, 2) === '0x') {
        hexString = hexString.substr(2);
    }
    
    let result = [];
    for (let i = 0; i < hexString.length; i += 2) {
        result.push(parseInt(hexString.substr(i, 2), 16));
    }
    return result;
}

  useEffect(() => {
    if (!currentAccount) return;
    let unsubscribe
    api.query.kittiesModule
      .ownerKitties(acctAddr(currentAccount), newValue => {
        // The storage value is an Option<u32>
        // So we have to check whether it is None first
        // There is also unwrapOr
        let kitties = [];
        if (newValue.isSome) {
          let value = newValue.unwrap().toHuman();
          for (let i = 0; i < value.length; i++) {
            api.query.kittiesModule.kitties(value[i], kitty => {
              kitties.push({
                id: value[i],
                dna: toByteArray(kitty.unwrap().toHuman()),
                owner: acctAddr(currentAccount)
              })
              if (kitties.length === value.length) {
                setKitties(kitties)
              }
            })
          }
        }
        else {
          setKitties(kitties)
        }
      })
      .then(unsub => {
        unsubscribe = unsub
      })
      .catch(console.error)

    return () => unsubscribe && unsubscribe()
  }, [api.query.kittiesModule, currentAccount])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={currentAccount} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          label='创建'
          type='SIGNED-TX'
          setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>

}
