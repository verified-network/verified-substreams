import "./App.css";
import { useSubstreams } from "./hooks/substream";
import { useMemo, useState } from "react";

function App() {
  const [block, setBlock] = useState(null);

  const { messages, connected, cursor } = useSubstreams(
    "https://sepolia.substreams.pinax.network:443",
    process.env.REACT_APP_SUBSTREAMS_API_TOKEN,
    "/secondarypool.spkg",
    "map_trades_per_pool",
    9496609,
    "/secondarypool.proto",
    "contract.v1.SecondaryPoolTradeReportsList"
  );

  const groupedByPool = useMemo(() => {
    const groups = {};

    for (const msg of messages) {
      for (const poolTrade of msg?.items[0]?.items || []) {
        const poolAddr = msg?.items[0].poolAddress;

        if (!groups[poolAddr]) groups[poolAddr] = { trades: [] };
        const formattedSub = {
          evtTxHash: "0x" + poolTrade.evtTxHash,
          evtIndex: poolTrade.evtIndex,
          evtBlockNumber: poolTrade.evtBlockNumber,
          evtAddress: "0x" + poolTrade.evtAddress,
          security: "0x" + Buffer.from(poolTrade.security).toString("hex"),
          currency: "0x" + Buffer.from(poolTrade.currency).toString("hex"),
          orderRef: "0x" + Buffer.from(poolTrade.orderRef).toString("hex"),
          orderType: "0x" + Buffer.from(poolTrade.orderType).toString("hex"),
          party: "0x" + Buffer.from(poolTrade.party).toString("hex"),
          counterparty:
            "0x" + Buffer.from(poolTrade.counterparty).toString("hex"),
          subscription: poolTrade.subscription,
          price: poolTrade.price,
          amount: poolTrade.amount,
          executionDate: new Date(Number(poolTrade.executionDate) * 1000),
        };

        groups[poolAddr].trades.push(formattedSub);

        setBlock(poolTrade.evtBlockNumber);
      }
    }

    return groups;
  }, [messages]);

  return (
    <div className="App">
      <header className="header">
        <h1>Secondary Pool Substream</h1>
        <p>
          Status: {"  "}
          <span className={`status-dot ${connected ? "connected" : ""}`}></span>
          {"  "}
          {connected ? "Connected" : "Disconnected"}
        </p>
        <p>From Block: 6589966 | Last Block: {block || "loading..."}</p>
      </header>

      <main className="content">
        {Object.keys(groupedByPool).length === 0 && (
          <p className="placeholder">Waiting for pool data…</p>
        )}

        {Object.entries(groupedByPool).map(([poolAddr, data]) => (
          <div key={poolAddr} className="pool-section">
            <h2 className="pool-address">0x{poolAddr}</h2>

            {/* Subscription Events */}
            {data.trades.map((ev, i) => (
              <div key={i} className="event-card">
                <h4 className="subheader">Trade</h4>
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
