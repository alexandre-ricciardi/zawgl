// deno-lint-ignore-file no-explicit-any
import { WebSocketClient, StandardWebSocketClient } from "https://deno.land/x/websocket@v0.1.3/mod.ts";
import { Bson } from "https://deno.land/x/bson/mod.ts";
import { concat } from "https://deno.land/std/bytes/mod.ts";

class ZawglClient {
    address: string;
    ws: WebSocketClient;
    query_promises: Map<string, Promise<Bson.Document>[]>;
    responses_resolvers: Map<string, any[]>;
    messages_dispatcher = (message: any) => {
      message.data.arrayBuffer().then((buffer: any) => {
        const doc = Bson.deserialize(buffer);
        const reqId: string = doc['request_id'];
        const resolvers = this.responses_resolvers.get(reqId);
        if (resolvers) {
          const resolver = resolvers.shift();
          if (resolver) {
            resolver(doc['graph']);
          }
        }
      });
    };

    constructor(address: string) {
        this.address = 'ws://' + address;
        this.ws = new StandardWebSocketClient(this.address);
        this.query_promises = new Map();
        this.responses_resolvers = new Map();
        this.ws.on("message", this.messages_dispatcher);
    }

    async executeCypherQuery(query: string): Promise<Bson.Document> {
      const reqId = crypto.randomUUID();
      const ogQuery = Bson.serialize({
        'request_id': reqId,
        'query': query,
      })
      const oneCyperHeader = "!application/openCypher";
      const headerBytes = Uint8Array.from([...oneCyperHeader].map(c => c.charCodeAt(0)));
      const res = new Promise<Bson.Document>((resolve) => {
        const resolvers = this.responses_resolvers.get(reqId);
        if (resolvers) {
          resolvers.push(resolve);
        } else {
          this.responses_resolvers.set(reqId, [resolve]);
        }
      });
      this.ws.send(concat(headerBytes, ogQuery));
      return await res;
    }
}

export { ZawglClient } ;