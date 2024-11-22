---
id: Pieces
title: Pieces
sidebar_position: 2
---

## Project architecture

Let's begin by looking at the pieces in the game.
## Class diagram

### Pieces

```mermaid
classDiagram

class PieceType{
    +authorized_positions()
    +protected_positions()
    +piece_to_utf_enum()
    +piece_to_fen_enum()
    +piece_type_to_string_enum()
    +partial_cmp()
    +cmp()
}

class PieceColor {
    <<enumeration>>
    Black
    White
    +opposite() PieceColor
}

class Pawn {
    +authorized_positions()
    +protected_positions()
    +to_string()
    +piece_move()
}

class Rook {
    +authorized_positions()
    +protected_positions()
    +to_string()
    +piece_move()
}

class Knight {
    +authorized_positions()
    +protected_positions()
    +to_string()
    +piece_move()
}

class Bishop {
    +authorized_positions()
    +protected_positions()
    +to_string()
    +piece_move()
}

class Queen {
    +authorized_positions()
    +protected_positions()
    +to_string()
    +piece_move()
}

class King {
    +authorized_positions()
    +protected_positions()
    +to_string()
    +piece_move()
}

class Coord {
    <<data structure>>
    +row: u8
    +col: u8
}


class PieceMove {
    +piece_type: PieceType
    +piece_color: PieceColor
    +from: Coord
    +to: Coord
}

class Movable {
    <<interface>>
    +piece_move()
}


class Position {
    <<interface>>
    +authorized_positions()
    +protected_positions()
}

PieceType <|-- Pawn
PieceType <|-- Rook
PieceType <|-- Bishop
PieceType <|-- Knight
PieceType <|-- King
PieceType <|-- Queen

PieceMove --> PieceType
PieceMove --> PieceColor
PieceMove --> Coord

PieceType ..|> Movable
PieceType ..|> Position

Pawn --> Coord
Rook --> Coord
Knight --> Coord
Bishop --> Coord
Queen --> Coord
King --> Coord

Movable <|.. Pawn
Movable <|.. Rook
Movable <|.. Knight
Movable <|.. Bishop
Movable <|.. Queen
Movable <|.. King

Position <|.. Pawn
Position <|.. Rook
Position <|.. Knight
Position <|.. Bishop
Position <|.. Queen
Position <|.. King
```

This schema can be a little bit overwhelming but let's break it apart.

#### PieceType

This class is basically the parent class of all the pieces. It contains the methods that are common to all the pieces. Such as authorized positions, protected positions, etc.

#### PieceColor

This is an enum that contains the two colors of the pieces. Black and White.

#### Pawn, Rook, Knight, Bishop, Queen, King

These are the classes that represent the pieces. They all inherit from PieceType and implement the methods that are specific to their type.

#### Movable and Position

These are rust traits that are implemented by the pieces. Movable is a trait that represents a piece that can move (piece_move method). Position is a trait that shows the authorized and protected positions of a piece.

#### Coord

This is a data structure that represents a position on the board. It contains a row and a column.

#### PieceMove

This is a data structure that represents a move of a piece. It contains the type of the piece, the color of the piece, the starting position, and the ending position. This is mainly used in the board structure that we will see later.

