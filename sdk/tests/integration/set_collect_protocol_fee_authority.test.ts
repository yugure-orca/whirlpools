import * as assert from "assert";
import * as anchor from "@project-serum/anchor";
import {
  WhirlpoolContext,
  AccountFetcher,
  WhirlpoolsConfigData,
  WhirlpoolIx,
  toTx,
} from "../../src";
import { generateDefaultConfigParams } from "../utils/test-builders";

describe("set_collect_protocol_fee_authority", () => {
  const provider = anchor.Provider.local();
  anchor.setProvider(anchor.Provider.env());
  const program = anchor.workspace.Whirlpool;
  const ctx = WhirlpoolContext.fromWorkspace(provider, program);
  const fetcher = new AccountFetcher(ctx.connection);

  it("successfully set_collect_protocol_fee_authority", async () => {
    const {
      configInitInfo,
      configKeypairs: { collectProtocolFeesAuthorityKeypair },
    } = generateDefaultConfigParams(ctx);
    await toTx(ctx, WhirlpoolIx.initializeConfigIx(ctx.program, configInitInfo)).buildAndExecute();
    const newAuthorityKeypair = anchor.web3.Keypair.generate();
    await toTx(
      ctx,
      WhirlpoolIx.setCollectProtocolFeesAuthorityIx(ctx.program, {
        whirlpoolsConfig: configInitInfo.whirlpoolsConfigKeypair.publicKey,
        collectProtocolFeesAuthority: collectProtocolFeesAuthorityKeypair.publicKey,
        newCollectProtocolFeesAuthority: newAuthorityKeypair.publicKey,
      })
    )
      .addSigner(collectProtocolFeesAuthorityKeypair)
      .buildAndExecute();
    const config = (await fetcher.getConfig(
      configInitInfo.whirlpoolsConfigKeypair.publicKey
    )) as WhirlpoolsConfigData;
    assert.ok(config.collectProtocolFeesAuthority.equals(newAuthorityKeypair.publicKey));
  });

  it("fails if current collect_protocol_fee_authority is not a signer", async () => {
    const {
      configInitInfo,
      configKeypairs: { collectProtocolFeesAuthorityKeypair },
    } = generateDefaultConfigParams(ctx);
    await toTx(ctx, WhirlpoolIx.initializeConfigIx(ctx.program, configInitInfo)).buildAndExecute();

    await assert.rejects(
      toTx(
        ctx,
        WhirlpoolIx.setCollectProtocolFeesAuthorityIx(ctx.program, {
          whirlpoolsConfig: configInitInfo.whirlpoolsConfigKeypair.publicKey,
          collectProtocolFeesAuthority: collectProtocolFeesAuthorityKeypair.publicKey,
          newCollectProtocolFeesAuthority: provider.wallet.publicKey,
        })
      ).buildAndExecute(),
      /Signature verification failed/
    );
  });

  it("fails if invalid collect_protocol_fee_authority provided", async () => {
    const { configInitInfo } = generateDefaultConfigParams(ctx);
    await toTx(ctx, WhirlpoolIx.initializeConfigIx(ctx.program, configInitInfo)).buildAndExecute();

    await assert.rejects(
      toTx(
        ctx,
        WhirlpoolIx.setCollectProtocolFeesAuthorityIx(ctx.program, {
          whirlpoolsConfig: configInitInfo.whirlpoolsConfigKeypair.publicKey,
          collectProtocolFeesAuthority: provider.wallet.publicKey,
          newCollectProtocolFeesAuthority: provider.wallet.publicKey,
        })
      ).buildAndExecute(),
      /0x7dc/ // An address constraint was violated
    );
  });
});
