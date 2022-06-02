@0x93510266ddb79d58;

struct Chunk {
    id @0 :Text;
    timeSent @1 :Float64;
    timeOffset @2 :Int32;
    entries @3 :List(Entry);

    struct Entry {
        time @0 :Float64;

        union {
            can :group {
                id @1 :UInt32;
                channel @2 :Text;
                remote @3 :Bool;
                error @4 :Bool;
                length @5 :UInt8;
                data @6 :Data;
            }
            gps :group {
                longitude @7 :Float64;
                latitude @8 :Float64;
                speed @9 :Float64;
            }
        }
    }
}
