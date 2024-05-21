import * as anchor from "@coral-xyz/anchor";
import { Program, Idl } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";
import getterSetterIdl from "../target/idl/getter_setter.json";
import interactingProgramIdl from "../../interacting_program/target/idl/interacting_program.json";
import { GetterSetter } from "../target/types/getter_setter";
import { InteractingProgram } from "../../interacting_program/target/types/interacting_program";

const getterSetterIdlTyped = getterSetterIdl as Idl & { metadata: { address: string } };
const interactingProgramIdlTyped = interactingProgramIdl as Idl & { metadata: { address: string } };

describe("combined_programs", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const getterSetterProgram = new Program(getterSetterIdlTyped, new PublicKey(getterSetterIdlTyped.metadata.address), provider) as Program<GetterSetter>;
  const interactingProgram = new Program(interactingProgramIdlTyped, new PublicKey(interactingProgramIdlTyped.metadata.address), provider) as Program<InteractingProgram>;

  const baseAccount = Keypair.generate();

  it("Initializes the base account", async () => {
    const allowedProgram = new PublicKey("2q1iXZ2fHMMimgppsFfghRMdBGk3iT9qxDo84Ltk1fwt");

    await getterSetterProgram.methods.initialize(allowedProgram)
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([baseAccount])
      .rpc();

    const account = await getterSetterProgram.account.baseAccount.fetch(baseAccount.publicKey);
    expect(account.allowedProgram.toBase58()).to.equal(allowedProgram.toBase58());
    expect(account.isAccessGranted).to.be.false;
  });

  it("Grants and revokes access", async () => {
    const value = new anchor.BN(42);

    try {
      await interactingProgram.methods.setValue(value)
        .accounts({
          baseAccount: baseAccount.publicKey,
          getterSetterProgram: getterSetterProgram.programId,
          interactingProgram: interactingProgram.programId,
        })
        .rpc();
      throw new Error("Expected error but none was thrown");
    } catch (err) {
      expect(err.message).to.include("Access not granted");
    }

    await getterSetterProgram.methods.grantAccess()
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    await interactingProgram.methods.setValue(value)
      .accounts({
        baseAccount: baseAccount.publicKey,
        getterSetterProgram: getterSetterProgram.programId,
        interactingProgram: interactingProgram.programId,
      })
      .rpc();

    let storedValue = await getterSetterProgram.account.baseAccount.fetch(baseAccount.publicKey);
    expect(storedValue.value.toNumber()).to.equal(42);

    await getterSetterProgram.methods.revokeAccess()
      .accounts({
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    try {
      await interactingProgram.methods.setValue(value)
        .accounts({
          baseAccount: baseAccount.publicKey,
          getterSetterProgram: getterSetterProgram.programId,
          interactingProgram: interactingProgram.programId,
        })
        .rpc();
      throw new Error("Expected error but none was thrown");
    } catch (err) {
      expect(err.message).to.include("Access not granted");
    }
  });
});
