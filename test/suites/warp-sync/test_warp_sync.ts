import { beforeAll, describeSuite, expect } from "@moonwall/cli";
import { ApiPromise, Keyring } from "@polkadot/api";
import { u8aToHex, stringToHex } from '@polkadot/util';
import { decodeAddress } from "@polkadot/util-crypto";
import { getAuthorFromDigest } from "../../util/author";
import { signAndSendAndInclude, waitSessions } from "../../util/block";
import { getKeyringNimbusIdHex } from "../../util/keys";
import { getHeaderFromRelay } from "../../util/relayInterface";
import fs from "fs/promises";

describeSuite({
  id: "W01",
  title: "Zombie Tanssi Warp Sync Test",
  foundationMethods: "zombie",
  testCases: function ({ it, context, log }) {
    let paraApi: ApiPromise;
    let relayApi: ApiPromise;
    let container2000Api: ApiPromise;

    beforeAll(async () => {
      paraApi = context.polkadotJs("Tanssi");
      relayApi = context.polkadotJs("Relay");
      container2000Api = context.polkadotJs("Container2000");

      const relayNetwork = relayApi.consts.system.version.specName.toString();
      expect(relayNetwork, "Relay API incorrect").to.contain("rococo");

      const paraNetwork = paraApi.consts.system.version.specName.toString();
      const paraId1000 = (await paraApi.query.parachainInfo.parachainId()).toString();
      expect(paraNetwork, "Para API incorrect").to.contain("dancebox");
      expect(paraId1000, "Para API incorrect").to.be.equal("1000");

      const container2000Network = container2000Api.consts.system.version.specName.toString();
      const paraId2000 = (await container2000Api.query.parachainInfo.parachainId()).toString();
      expect(container2000Network, "Container2000 API incorrect").to.contain("container-chain-template");
      expect(paraId2000, "Container2000 API incorrect").to.be.equal("2000");

      // Test block numbers in relay are 0 yet
      const header2000 = await getHeaderFromRelay(relayApi, 2000);

      expect(header2000.number.toNumber()).to.be.equal(0);
    }, 120000);

    it({
      id: "T01",
      title: "Blocks are being produced on parachain",
      test: async function () {
        const blockNum = (await paraApi.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T03",
      title: "Test assignation did not change",
      test: async function () {
        const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
        // TODO: fix once we have types
        const allCollators = (await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)).toJSON();
        const expectedAllCollators = {
          orchestratorChain: [
            getKeyringNimbusIdHex("Collator1000-01"),
            getKeyringNimbusIdHex("Collator1000-02"),
            getKeyringNimbusIdHex("Collator1000-03"),
          ],
          containerChains: {
            "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator2000-02")],
          },
        };

        expect(allCollators).to.deep.equal(expectedAllCollators);
      },
    });

    it({
      id: "T04",
      title: "Blocks are being produced on container 2000",
      test: async function () {
        const blockNum = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
        expect(blockNum).to.be.greaterThan(0);
      },
    });

    it({
      id: "T06",
      title: "Test container chain 2000 assignation is correct",
      test: async function () {
        const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
        const paraId = (await container2000Api.query.parachainInfo.parachainId()).toString();
        const containerChainCollators = (
          await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)
        ).toJSON().containerChains[paraId];

        // TODO: fix once we have types
        const writtenCollators = (await container2000Api.query.authoritiesNoting.authorities()).toJSON();

        expect(containerChainCollators).to.deep.equal(writtenCollators);
      },
    });

    it({
      id: "T08",
      title: "Test author noting is correct for both containers",
      timeout: 60000,
      test: async function () {
        const assignment = await paraApi.query.collatorAssignment.collatorContainerChain();
        const paraId2000 = await container2000Api.query.parachainInfo.parachainId();

        // TODO: fix once we have types
        const containerChainCollators2000 = assignment.containerChains.toJSON()[paraId2000.toString()];

        await context.waitBlock(3, "Tanssi");
        const author2000 = await paraApi.query.authorNoting.latestAuthor(paraId2000);

        expect(containerChainCollators2000.includes(author2000.toJSON().author)).to.be.true;
      },
    });

    it({
      id: "T09",
      title: "Test author is correct in Orchestrator",
      test: async function () {
        const sessionIndex = (await paraApi.query.session.currentIndex()).toNumber();
        const authorities = await paraApi.query.authorityAssignment.collatorContainerChain(sessionIndex);
        const author = await getAuthorFromDigest(paraApi);
        // TODO: fix once we have types
        expect(authorities.toJSON().orchestratorChain.includes(author.toString())).to.be.true;
      },
    });

    it({
      id: "T10",
      title: "Test frontier template isEthereum",
      test: async function () {
        // TODO: fix once we have types
        const genesisData2000 = await paraApi.query.registrar.paraGenesisData(2000);
        expect(genesisData2000.toJSON().properties.isEthereum).to.be.false;
      },
    });

    it({
      id: "T12",
      title: "Test warp sync: collator rotation from tanssi to container with blocks",
      timeout: 300000,
      test: async function () {
        const keyring = new Keyring({ type: "sr25519" });
        let alice = keyring.addFromUri("//Alice", { name: "Alice default" });

        // Collator2000-02 should have a container 2000 db, and Collator1000-03 should not
        const collator100003DbPath = getTmpZombiePath() + "/Collator1000-03/data/containers/chains/simple_container_2000/db/full-container-2000";
        const container200002DbPath = getTmpZombiePath() + "/Collator2000-02/data/containers/chains/simple_container_2000/db/full-container-2000";
        expect(await directoryExists(container200002DbPath)).to.be.true;
        expect(await directoryExists(collator100003DbPath)).to.be.false;        

        // Deregister Collator2000-02, it should delete the db
        const invuln = (await paraApi.query.collatorSelection.invulnerables()).toJSON();

        const newInvuln = invuln.filter((addr) => {
          return u8aToHex(decodeAddress(addr)) != getKeyringNimbusIdHex("Collator2000-02");
        });
        // It must have changed
        expect(newInvuln).to.not.deep.equal(invuln);

        const tx = paraApi.tx.collatorSelection.setInvulnerables(newInvuln);
        await signAndSendAndInclude(paraApi.tx.sudo.sudo(tx), alice);

        await waitSessions(context, paraApi, 2);

        // Collator1000-03 should rotate to container chain 2000

        const currentSession = (await paraApi.query.session.currentIndex()).toNumber();
        // TODO: fix once we have types
        const allCollators = (await paraApi.query.authorityAssignment.collatorContainerChain(currentSession)).toJSON();
        const expectedAllCollators = {
          orchestratorChain: [
            getKeyringNimbusIdHex("Collator1000-01"),
            getKeyringNimbusIdHex("Collator1000-02"),
          ],
          containerChains: {
            "2000": [getKeyringNimbusIdHex("Collator2000-01"), getKeyringNimbusIdHex("Collator1000-03")],
          },
        };

        expect(allCollators).to.deep.equal(expectedAllCollators);

        // Collator2000-02 container chain db should have been deleted
        expect(await directoryExists(container200002DbPath)).to.be.false;

        // Collator1000-03 container chain db should be created
        expect(await directoryExists(collator100003DbPath)).to.be.true;
      },
    });

    it({
      id: "T13",
      title: "Collator1000-03 is producing blocks on Container 2000",
      test: async function () {
        const blockStart = (await container2000Api.rpc.chain.getBlock()).block.header.number.toNumber();
        // Wait up to 8 blocks, giving the new collator 4 chances to build a block
        const blockEnd = blockStart + 8;
        const authors = [];

        for (let blockNumber = blockStart; blockNumber <= blockEnd; blockNumber += 1) {
            // Get the latest author from Digest
            const blockHash = await container2000Api.rpc.chain.getBlockHash(blockNumber);
            const apiAt = await container2000Api.at(blockHash);
            const digests = (await apiAt.query.system.digest()).logs;
            const filtered = digests.filter(log => 
                log.isPreRuntime === true && log.asPreRuntime[0].toHex() == stringToHex('nmbs')
            );
            const author = filtered[0].asPreRuntime[1].toHex();
            authors.push(author);
            if (author == getKeyringNimbusIdHex("Collator1000-03")) {
                break;
            }
            await context.waitBlock(1, "Tanssi");
        }

        expect(authors).to.contain(getKeyringNimbusIdHex("Collator1000-03"))
      },
    });
  },
});

async function directoryExists(directoryPath) {
  try {
      await fs.access(directoryPath, fs.constants.F_OK);
      return true;
  } catch (err) {
      return false;
  }
}

/// Returns the /tmp/zombie-52234... path
function getTmpZombiePath() {
  const logFilePath = process.env.MOON_MONITORED_NODE;

  if (logFilePath) {
    const lastIndex = logFilePath.lastIndexOf('/');
    return lastIndex !== -1 ? logFilePath.substring(0, lastIndex) : null;
  }

  // Return null if the environment variable is not set
  return null;
}
