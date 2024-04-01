import { useState } from 'react'
import './App.css'
import {
    createConnectTransport,
    createPromiseClient,
    ConnectError,
    codeToString,
} from "@bufbuild/connect-web";

// Import service definition that you want to connect to.
import { Kv } from "../gen/read_connectweb";
import { Subscription } from "../gen/primary_pb";


// The transport defines what type of endpoint we're hitting.
// In our example we'll be communicating with a Connect endpoint.
const transport = createConnectTransport({
    baseUrl: "http://localhost:8000",
});

// Here we make the client itself, combining the service
// definition with the transport.
const client = createPromiseClient(Kv, transport);

function App() {
    const [inputValue, setInputValue] = useState("");
    const [messages, setMessages] = useState<
        {
            message: string;
            color: string;
        }[]
    >([]);
    return <>
        <h1>Example UI for Primary and Secondary pool substreams</h1>
        <h2>Enter a key to get the value from the kv store (ex: month:last:201511)</h2>
        <ol>
            {messages.map((msg, index) => (
                <li key={index}>
                    <div style={{ color: msg.color }}>
                        <pre>{msg.message}</pre>
                    </div>
                </li>
            ))}
        </ol>
        <form onSubmit={async (e) => {
            e.preventDefault();
            // Clear inputValue since the user has submitted.
            setInputValue("");
            setMessages((prev) => [
                ...prev,
                {
                    message: "Request: " + inputValue,
                    color: "grey",
                },
            ]);
            try {
                const response = await client.get({
                    key: inputValue.replace("0x", ""),
                });
                let output: string;
                try {
                    const subscription = Subscription.fromBinary(response.value);
                    output = JSON.stringify(subscription, (key, value) => {
                        if (key === "assetInAddress") {
                            return "0x" + bufferToHex(subscription.assetInAddress);
                        }
                        if (key === "assetOutAddress") {
                            return "0x" + bufferToHex(subscription.assetOutAddress);
                        }
                        if (key === "investorAddress") {
                            return "0x" + bufferToHex(subscription.investorAddress);
                        }
                        if (key === "price") {
                            return "0x" + bigintToHex(subscription.price);
                        }
                        if (key === "subscriptionAmount") {
                            return "0x" + bigintToHex(subscription.subscriptionAmount);
                        }
                        if (key === "executionDate") {
                            return "0x" + bigintToHex(subscription.executionDate);
                        }
                        return value;
                    }, 6);
                } catch (e) {
                    output = "0x" + bufferToHex(response.value);
                }
                setMessages((prev) => [
                    ...prev,
                    {
                        message: output,
                        color: "lightblue",
                    },
                ]);
            } catch (e) {
                let errorMessage: string;

                if (e instanceof ConnectError) {
                    errorMessage = JSON.stringify({
                        name: e.name,
                        code: codeToString(e.code),
                        message: e.rawMessage,
                    }, null, 2);
                } else {
                    errorMessage = JSON.stringify(e, null, 2);
                }
                setMessages((prev) => [
                    ...prev,
                    {
                        message: "Error: " + errorMessage,
                        color: "pink",
                    },
                ]);

            }
        }}>
            <input value={inputValue} onChange={e => setInputValue(e.target.value)} />
            <button type="submit">Send</button>
        </form>
    </>;
}


function bufferToHex(buffer: Uint8Array): string {
    var s = '', h = '0123456789abcdef';
    (new Uint8Array(buffer)).forEach((v) => { s += h[v >> 4] + h[v & 15]; });
    return s;
}
function bigintToHex(bigintValue: bigint): string {
    if (typeof bigintValue !== 'bigint') {
        throw new Error('Input must be a bigint.');
    }

    const h = '0123456789abcdef';
    let s = '';
    let value = bigintValue;

    while (value > 0n) {
        const digit = Number(value & 15n);
        s = h[digit] + s;
        value >>= 4n;
    }

    return s;
}
export default App