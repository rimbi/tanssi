import "@tanssi/api-augment";
import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";
import { ApiPromise } from "@polkadot/api";
import { jumpSessions } from "../../../util/block";

describeSuite({
  id: "DT0502",
  title: "Registrar test suite",
  foundationMethods: "dev",
  testCases: ({ it, context, log }) => {
    let polkadotJs: ApiPromise;
    let alice: KeyringPair;

    beforeAll(() => {
      alice = context.keyring.alice;
      polkadotJs = context.polkadotJs();
    });

    it({
      id: "E01",
      title: "Checking that fetching registered paraIds is possible",
      test: async function () {
        const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();

        // These are registered in genesis
        // TODO: fix once we have types
        expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001]);
      },
    });

    it({
      id: "E02",
      title: "Checking that registering paraIds is possible",
      test: async function () {
        await context.createBlock();

        const currentSesssion = await polkadotJs.query.session.currentIndex();
        const sessionDelay = await polkadotJs.consts.registrar.sessionDelay;
        const expectedScheduledOnboarding = BigInt(currentSesssion.toString()) + BigInt(sessionDelay.toString());

        const emptyGenesisData = () => {
          let g = polkadotJs.createType("TpContainerChainGenesisDataContainerChainGenesisData", {
            storage: [
              {
                key: "0x636f6465",
                value: "0x010203040506",
              },
            ],
            name: "0x436f6e7461696e657220436861696e2032303030",
            id: "0x636f6e7461696e65722d636861696e2d32303030",
            forkId: null,
            extensions: "0x",
            properties: {
              tokenMetadata: {
                tokenSymbol: "0x61626364",
                ss58Format: 42,
                tokenDecimals: 12,
              },
              isEthereum: false,
            },
          });
          return g;
        };
        const containerChainGenesisData = emptyGenesisData();

        const tx = polkadotJs.tx.registrar.register(2002, containerChainGenesisData);
        const tx2 = polkadotJs.tx.registrar.markValidForCollating(2002);
        const nonce = await polkadotJs.rpc.system.accountNextIndex(alice.publicKey);
        await context.createBlock([
          await tx.signAsync(alice, { nonce }),
          await polkadotJs.tx.sudo.sudo(tx2).signAsync(alice, { nonce: nonce.addn(1) }),
        ]);

        const pendingParas = await polkadotJs.query.registrar.pendingParaIds();
        expect(pendingParas.length).to.be.eq(1);
        const sessionScheduling = pendingParas[0][0];
        const parasScheduled = pendingParas[0][1];

        expect(sessionScheduling.toBigInt()).to.be.eq(expectedScheduledOnboarding);

        // These will be the paras in session 2
        // TODO: fix once we have types
        expect(parasScheduled.toJSON()).to.deep.equal([2000, 2001, 2002]);

        // Check that the on chain genesis data is set correctly
        const onChainGenesisData = await polkadotJs.query.registrar.paraGenesisData(2002);
        // TODO: fix once we have types
        expect(emptyGenesisData().toJSON()).to.deep.equal(onChainGenesisData.toJSON());

        // Checking that in session 2 paras are registered
        await jumpSessions(context, 2);

        // Expect now paraIds to be registered
        const parasRegistered = await polkadotJs.query.registrar.registeredParaIds();
        // TODO: fix once we have types
        expect(parasRegistered.toJSON()).to.deep.equal([2000, 2001, 2002]);
      },
    });
  },
});
