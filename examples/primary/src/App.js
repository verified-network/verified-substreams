import "./App.css";
import { useSubstreams } from "./hooks/substream";
import { useMemo, useState } from "react";

function App() {
  const [block, setBlock] = useState(null);

  const { messages, connected, cursor } = useSubstreams(
    "https://sepolia.substreams.pinax.network:443",
    process.env.REACT_APP_SUBSTREAMS_API_TOKEN,
    "/primary_pool.spkg",
    "map_subscriptions_per_pool",
    5492879,
    "/primary_pool.proto",
    "contract.v1.PrimaryPoolSubscriptionsList"
  );

  const groupedByPool = useMemo(() => {
    const groups = {};

    for (const msg of messages) {
      for (const poolSubs of msg?.items[0]?.items || []) {
        const poolAddr = msg?.items[0].poolAddress;

        if (!groups[poolAddr]) groups[poolAddr] = { subscriptions: [] };
        const formattedSub = {
          evtTxHash: "0x" + poolSubs.evtTxHash,
          evtIndex: poolSubs.evtIndex,
          evtBlockNumber: poolSubs.evtBlockNumber,
          evtAddress: "0x" + poolSubs.evtAddress,
          assetIn: "0x" + Buffer.from(poolSubs.assetIn).toString("hex"),
          assetOut: "0x" + Buffer.from(poolSubs.assetOut).toString("hex"),
          investor: "0x" + Buffer.from(poolSubs.investor).toString("hex"),
          subscription: poolSubs.subscription,
          price: poolSubs.price,
          executionDate: new Date(Number(poolSubs.executionDate) * 1000),
        };

        groups[poolAddr].subscriptions.push(formattedSub);

        setBlock(poolSubs.evtBlockNumber);
      }
    }

    return groups;
  }, [messages]);

  console.log(
    "messages: ",
    messages[0]?.items,
    messages[0]?.items[0]?.items,
    typeof messages[0]?.items
  );

  return (
    <div className="App">
      <header className="header">
        <h1>Primary Pool Substream</h1>
        <p>
          Status: {"  "}
          <span className={`status-dot ${connected ? "connected" : ""}`}></span>
          {"  "}
          {connected ? "Connected" : "Disconnected"}
        </p>
        <p>From Block: 5492879 | Last Block: {block || "loading..."}</p>
      </header>

      <main className="content">
        {Object.keys(groupedByPool).length === 0 && (
          <p className="placeholder">Waiting for pool data…</p>
        )}

        {Object.entries(groupedByPool).map(([poolAddr, data]) => (
          <div key={poolAddr} className="pool-section">
            <h2 className="pool-address">0x{poolAddr}</h2>

            {/* Subscription Events */}
            {data.subscriptions.map((ev, i) => (
              <div key={i} className="event-card">
                <h4 className="subheader">Subscription</h4>
                <pre className="json-block">{JSON.stringify(ev, null, 2)}</pre>
              </div>
            ))}
          </div>
        ))}
      </main>
    </div>
  );
}

export default App;
