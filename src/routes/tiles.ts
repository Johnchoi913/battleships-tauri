export type Tile =
    | { type: "Empty" }
    | { type: "Ship"; value: number }
    | { type: "Hit"; value: number }
    | { type: "Miss" }
    | { type: "Dead"; value: number };

export function createEmptyBoard(): Tile[][] {
    return Array.from({ length: 10 }, () =>
        Array.from({ length: 10 }, () => ({ type: "Empty" }))
    );
}