@0x93510266ddb79d58;

struct Chunk {
    id @0 :Text;
    time @1 :Float64;
    entries @2 :List(Entry);

    struct Entry {
        time @0 :Float64;

        union {
            can :group {
                id @1 :UInt32;
                channel @2 :Text;
                remote @3 :Bool;
                error @4 :Bool;
                extended @5 :Bool;
                length @6 :UInt8;
                data @7 :Data;
            }
            gps :group {
                longitude @8 :Float64;
                latitude @9 :Float64;
                speed @10 :Float64;
            }
        }
    }
}
