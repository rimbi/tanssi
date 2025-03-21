import { describeSuite, expect } from "@moonwall/cli";
import {
  ALITH_ADDRESS,
  ALITH_PRIVATE_KEY,
  BALTATHAR_ADDRESS,
  CHARLETH_ADDRESS,
  createRawTransfer,
  customWeb3Request
} from "@moonwall/util";

describeSuite({
  id: "DF0201",
  title: "Ethereum Block - Pending",
  foundationMethods: "dev",
  testCases: ({ context, it, log }) => {
    const TEST_ACCOUNT = "0x1111111111111111111111111111111111111111";

    it({
      id: "T01",
      title: "should return pending block",
      test: async function () {
        let nonce = 0;
        let sendTransaction = async () => {
          const tx = await context.web3().eth.accounts.signTransaction(
            {
              from: ALITH_ADDRESS,
              to: TEST_ACCOUNT,
              value: "0x200", // Must be higher than ExistentialDeposit
              gasPrice: "0x3B9ACA00",
              gas: "0x100000",
              nonce: nonce,
            },
            ALITH_PRIVATE_KEY
          );
          nonce = nonce + 1;
          return (await customWeb3Request(context.web3(), "eth_sendRawTransaction", [tx.rawTransaction])).result;
        };
    
        // block 1 send 5 transactions
        const expectedXtsNumber = 5;
        for (let _ of Array(expectedXtsNumber)) {
          await sendTransaction();
        }
    
        // test still invalid future transactions can be safely applied (they are applied, just not overlayed)
        nonce = nonce + 100;
        await sendTransaction();
    
        // do not seal, get pendign block
        let pending_transactions = [];
        {
          const pending = (await customWeb3Request(context.web3(), "eth_getBlockByNumber", ["pending", false])).result;
          expect(pending.hash).to.be.null;
          expect(pending.miner).to.be.null;
          expect(pending.nonce).to.be.null;
          expect(pending.totalDifficulty).to.be.null;
          pending_transactions = pending.transactions;
          expect(pending_transactions.length).to.be.eq(expectedXtsNumber);
        }
    
        // seal and compare latest blocks transactions with the previously pending
        await context.createBlock();
        const latest_block = await context.web3().eth.getBlock("latest", false);
        expect(pending_transactions).to.be.deep.eq(latest_block.transactions)
      },
    });
  },
});