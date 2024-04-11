export class DorcsSocket {
    private socket?: WebSocket;
    private url: string;
    private onMessage: (event: MessageEvent) => void;

    constructor(url: string, onMessage: (event: MessageEvent) => void) {
        this.url = url;
        this.onMessage = onMessage;
        this.connect();
    }

    private connect(): void {
        this.socket = new WebSocket(this.url);
        this.socket.onmessage = this.onMessage;
        this.socket.onopen = (): void => {
            console.log('Websocket connected.');
        }
    }
}