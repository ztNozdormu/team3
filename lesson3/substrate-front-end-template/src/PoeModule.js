import React, { useEffect, useState, createRef } from 'react';
import { CopyToClipboard } from 'react-copy-to-clipboard';
import { Form, Input, Grid,Menu,
  Button,
  Dropdown,
  Container,
  Icon,
  Image,
  Label, Sticky} from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import {blake2AsHex} from '@polkadot/util-crypto';
import AccountSelector from './AccountSelector';
import Transfer from './Transfer';
// import { userInfo } from 'os';

const contextRef = createRef();
// main组件
function Main (props) {
  // 获取API接口对象
  const { api } = useSubstrate();
  // 包含账号交易信息
  const { accountPair } = props;

  // The transaction submission status 使用useState生成一个新的status属性
  const [status, setStatus] = useState('');
  const [digest,setDigest] = useState('');
  const [owner,setOwner] = useState('');
  const [blockNumber,setBlockNumber] = useState(0);

  useEffect(() => {
    let unsubscribe;
    // 检查poeModule模块的proofs存储是否有更新，通过digest去查对应的数据result
    api.query.poeModule.proofs(digest,(result) => {
      // The storage value is an Option<u32> 
      setOwner(result[0].toString());
      console.log(result);
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);
    // 清理组件状态的时候解除监听事件
    return () => unsubscribe && unsubscribe();
  }, [digest,api.query.poeModule]);
  // 定义存证文件选中方法
  const handleFileChoose = (file) =>{
    let fileReader = new FileReader();
    // 将文件二进制数据转换为sha256字符串
    const bufferToDigest = () =>{
      const content = Array.from(new Uint8Array(fileReader.result))
      .map((b)=>b.toString(16).padStart(2,'0')).join('');// 不足两位补零
     // 计算文件hash
     const fileHash= blake2AsHex(content,256);
     setDigest(fileHash);
    }
    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }
  // ========================账号操作相关js===================================
  // 选择的接收存证人账号
  const [accountAddress, setAccountAddress] = useState(null);
  // ========================账号操作相关js===================================
  // ========================购买存证========================================
  const [formState, setFormState] = useState({ sellPrice: 1000, buyPrice: 0 });
  const onChange = (_, data) =>
    setFormState(prev => ({ ...prev, [data.state]: data.value }));
  const { sellPrice, buyPrice } = formState;

  // ========================购买存证========================================
  // 自定义UI组件
  return (
    <Grid.Column width={8}>
        {/* ===========存证UI======= */}
      <h1>Proofs of Existence Module</h1>
      <Form>
        {/*存证源文件--选择功能组件 */}
        <Form.Field>
          <input type='file' id='file' label='select existtence file' onChange={(e) => handleFileChoose(e.target.files[0])}>
          </input>
        </Form.Field>
        <Sticky context={contextRef}>
        <AccountSelector setAccountAddress={setAccountAddress} />
        </Sticky>
        <Form.Field>
          {/* 创建存证功能按钮 */}
          <TxButton 
            accountPair={accountPair} 
            label='创建存证'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
                  //  调用的模块
                  palletRpc:'poeModule',
                  callable:'createClaim',
                  //  需要输入的参数
                  inputParams:[digest],
                  paramFields:[true]
                }}
          />
          {/* 撤销存证功能按钮 */}
          <TxButton
              accountPair={accountPair}
              label='撤销存证'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={{
                  palletRpc:'poeModule',
                  callable:'revokeClaim',
                  inputParams:[digest],
                  paramFields:[true]
              }}
           />
           {/* 转移存证 */}
           <TxButton 
              accountPair={accountPair}
              label='转移存证'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={{
                palletRpc:'poeModule',
                callable:'transferClaim',
                inputParams:[digest,accountAddress],
                paramFields:[true]
              }}
           />
           {/* 卖方设置存证价格 */}
           <Input
            fluid
            label='卖方出价'
            type='number'
            state='sellPrice'
            onChange={onChange}
          />
          <TxButton 
              accountPair={accountPair}
              label='设置存证价格'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={{
                palletRpc:'poeModule',
                callable:'setPrice',
                inputParams:[digest,sellPrice],
                paramFields:[true,true]
              }}
           />
        </Form.Field>
          {/* 购买存证 */}
        <Form.Field>
          <Input
            fluid
            label='买方出价'
            type='number'
            state='buyPrice'
            onChange={onChange}
          />
        </Form.Field>
        <TxButton 
              accountPair={accountPair}
              label='购买存证(默认当前操作人)'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={{
                palletRpc:'poeModule',
                callable:'buyClaim',
                inputParams:[digest,buyPrice],
                paramFields:[true,true]
              }}
           />
         {/* 操作信息提示 */}
            <div>{status}</div>
            <div>{`Claim info , owner is ${owner},baockNumber is ${blockNumber}`}</div>
            <div>{`Claim info , owner is ${owner},baockNumber is ${blockNumber},accepter is ${accountAddress}`}</div>
            <div>{`Claim info , seller: ${owner},set the sellPrice is ${sellPrice}`}</div>
            <div>{`Claim info , buyer: ${owner},set buy the claim in block:${blockNumber}`}</div>
      </Form>
      {/* ===========存证UI======= */}

    </Grid.Column>
    
  );
}

export default function PoeModule (props) {
  // 返回区块链api对象
  const { api } = useSubstrate();
  // 判断是否存在poeModule模块 以及模块的存储是否存在 则渲染main组件
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
