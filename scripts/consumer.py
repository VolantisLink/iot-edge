import sys
import os
import io
import click
import capnp
import paho.mqtt.client as mqtt

from pathlib import Path

current_dir = Path(os.path.abspath(__file__))
SCHEMA = current_dir.absolute().parent.joinpath('..', 'schema', 'chunk.capnp')

capnp.remove_import_hook()
chunk = capnp.load(os.path.abspath(SCHEMA))
print(dir(chunk.Chunk))

def run(host, port, topic):
    def on_connect(client, userdata, flags, rc):
        print("Connected with result code "+str(rc))
        client.subscribe(topic)

    def on_message(client, userdata, data):
        msg = chunk.Chunk.from_bytes_packed(data.payload)
        print(msg.to_dict())

    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message

    client.connect(host, port, 60)

    client.loop_forever()

@click.command()
@click.option('--host', default='localhost', help='mqtt broker host')
@click.option('--port', default=1883)
@click.option('--topic', default='hello/test')
def main(host, port, topic):
    """Simple program for test."""
    run(host, port, topic)

if __name__ == '__main__':
    main()