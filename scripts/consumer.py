import os
import datetime
import cantools
import click
import capnp
import paho.mqtt.client as mqtt

from pathlib import Path

current_dir = Path(os.path.abspath(__file__))
SCHEMA = current_dir.absolute().parent.joinpath('..', 'schema', 'chunk.capnp')
DBC = current_dir.absolute().parent.joinpath('..', 'dbc', 'bmw_e9x_e8x.dbc')

capnp.remove_import_hook()
schema = capnp.load(os.path.abspath(SCHEMA))
db = cantools.database.load_file(DBC)

def run(host, port, topic):
    def on_connect(client, userdata, flags, rc):
        print("Connected with result code "+str(rc))
        client.subscribe(topic)

    def on_message(client, userdata, data):
        chunk = schema.Chunk.from_bytes_packed(data.payload)
        print(chunk.to_dict())            

    client = mqtt.Client()
    client.on_connect = on_connect
    client.on_message = on_message

    client.connect(host, port, 60)

    client.loop_forever()

@click.command()
@click.option('--host', default='localhost', help='mqtt broker host')
@click.option('--port', default=1883)
@click.option('--topic', default='hello/test/capnp')
def main(host, port, topic):
    """Simple program for test."""
    run(host, port, topic)

if __name__ == '__main__':
    main()