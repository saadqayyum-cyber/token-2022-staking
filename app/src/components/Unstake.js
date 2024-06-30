import { useEffect, useState } from "react";
import { AnchorError, AnchorProvider, Program } from "@coral-xyz/anchor";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import idl from "../idl/idl.json";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import Swal from "sweetalert2";
import { Audio } from "react-loader-spinner";

// const STAKING_CONTRACT_ADDRESS = process.env.REACT_APP_STAKING_CONTRACT_ADDRESS;
// const TOKEN_ADDRESS = process.env.REACT_APP_TOKEN_ADDRESS;
// const programID = STAKING_CONTRACT_ADDRESS || null;
// const tokenMintAddress = TOKEN_ADDRESS ? new PublicKey(TOKEN_ADDRESS) : null;

function Unstake() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const provider = new AnchorProvider(connection, wallet, "processed");

  const [yourStaked, setYourStaked] = useState(0);
  const [isLoading, setIsLoading] = useState(false);

  // let program = null;

  // if (programID) {
  //   program = new Program(idl, programID, provider);
  // }

  const handleUnstake = async () => {};

  const calculateRewards = async () => {};

  return (
    <div className="flex items-center justify-center h-screen px-4">
      <div className="bg-purple-light rounded-lg p-6 w-full max-w-md mx-auto">
        <div className="flex justify-between items-center">
          <h2 className="text-xl font-semibold">Unstake</h2>
        </div>
        <div className="mt-4 text-sm">
          <p>
            Unstaking will release your staked tokens and simultaneously claim any accrued rewards. This streamlined process
            ensures that you quickly regain access to your tokens and receive all rewards you are entitled to, based on the
            duration and amount of your stake.
          </p>
        </div>

        <div className="mt-4 space-y-2">
          <div className="bg-purple-medium p-4 rounded-lg">
            <div className="flex justify-between">
              <span>Your Total Staked</span>
              {/* <span>~0.00 SOL</span> */}
            </div>
            <div className="text-3xl font-bold">{yourStaked} TOKENS</div>
          </div>

          <div className="bg-purple-medium p-4 rounded-lg">
            <div className="flex justify-between">
              <span>Rewards</span>
              {/* <span>~0.00 SOL</span> */}
            </div>
            <div className="text-3xl font-bold">{yourStaked} TOKENS</div>
          </div>

          <div className="mt-4"></div>
          <div className="mt-6 flex flex-col sm:flex-row justify-between space-y-4 sm:space-y-0">
            {isLoading ? (
              <Audio height="35" width="35" radius="9" color="yellow" ariaLabel="loading" wrapperStyle wrapperClass />
            ) : (
              <button className="bg-yellow-400 text-black px-6 py-2 rounded-lg" onClick={handleUnstake}>
                UNSTAKE
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default Unstake;
