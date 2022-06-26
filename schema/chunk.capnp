@0x93510266ddb79d58;

struct CanMessage {
    time @0 :Float64;
    id @1 :UInt32;
    channel @2 :Text;
    remote @3 :Bool;
    error @4 :Bool;
    extended @5 :Bool;
    length @6 :UInt8;
    data @7 :Data;
}

struct GpsMessage {
    time @0 :Float64;
    longitude @1 :Float64;
    latitude @2 :Float64;
    speed @3 :Float64;
}

struct Chunk {
    id @0 :Text;
    time @1 :Float64;
    can @2 :List(CanMessage);
    gps @3 :List(GpsMessage);
}
