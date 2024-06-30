import { AnchorError, AnchorProvider, Program } from "@coral-xyz/anchor";
import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { useEffect, useMemo, useState } from "react";
import idl from "../idl/idl.json";
import Countdown from "react-countdown";
import { Audio } from "react-loader-spinner";
import Swal from "sweetalert2";
import dayjs from "dayjs";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

function ClaimRewards() {
  const { connection } = useConnection();
  const wallet = useWallet();
  const provider = new AnchorProvider(connection, wallet, "processed");

  const [yourRewards, setYourRewards] = useState(0);
  const [rewardsInSol, setRewardsInSol] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  const [isCalculateLoading, setIsCalculateLoading] = useState(false);
  const [leftLockPeriod, setLeftLockPeriod] = useState();
  const [fetchDataTrigger, setFetchDataTrigger] = useState(false);
  const [countdownKey, setCountdownKey] = useState(0);

  const calculateRewards = async (liveShidonkSolPrice) => {};

  const claimRewards = async () => {};

  useEffect(() => {
    setCountdownKey((prevKey) => prevKey + 1);
  }, [leftLockPeriod]);

  return (
    <div className="flex items-center justify-center h-screen px-4">
      <div className="bg-purple-light rounded-lg p-6 w-full max-w-md mx-auto">
        <div className="flex justify-between items-center">
          <h2 className="text-xl font-semibold">Claim Rewards</h2>
        </div>
        <div className="mt-4 text-sm">
          <p>
            Track and claim your rewards instantly! Our platform updates your earnings in real time at a 15% APY, allowing you to
            see and manage your rewards effortlessly. Just one click, and it's yours—simple and hassle-free.
          </p>
        </div>

        <div className="mt-4">
          <div className="bg-purple-medium p-4 rounded-lg">
            <div className="flex justify-between">
              <span>Your Rewards</span>
              <span>~{rewardsInSol?.toFixed(20)} SOL</span>
            </div>
            <div className="text-3xl font-bold">{yourRewards?.toFixed(8)} TOKENS</div>
          </div>

          <div className="bg-purple-medium p-4 rounded-lg mt-3">
            <div className="flex justify-between">
              <span>Unlock Period</span>
            </div>

            <div className="text-3xl font-bold">
              {+yourRewards !== 0 && <Countdown key={countdownKey} autoStart date={leftLockPeriod} />}
            </div>
          </div>

          <div className="mt-4"></div>
          <div className="mt-6 flex flex-col sm:flex-row justify-between space-y-4 sm:space-y-0">
            {isCalculateLoading ? (
              <Audio height="35" width="35" radius="9" color="yellow" ariaLabel="loading" wrapperStyle wrapperClass />
            ) : (
              <button
                className="bg-yellow-400 text-black px-6 py-2 rounded-lg"
                onClick={() => setFetchDataTrigger((prevState) => !prevState)}
              >
                CALCULATE
              </button>
            )}

            {isLoading ? (
              <Audio height="35" width="35" radius="9" color="yellow" ariaLabel="loading" wrapperStyle wrapperClass />
            ) : (
              <button className="bg-yellow-400 text-black px-6 py-2 rounded-lg" onClick={claimRewards}>
                CLAIM
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default ClaimRewards;
