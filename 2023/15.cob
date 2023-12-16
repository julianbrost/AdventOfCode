      * Note: I was unable to read lines longer than 8192 bytes, so the commas
      * in the input have to by replaced by newlines, like this:
      *
      * tr , '\n' < 15.in | ./15

       IDENTIFICATION DIVISION.
       PROGRAM-ID. day15.
       DATA DIVISION.
       WORKING-STORAGE SECTION.
       01 Seq PIC X(128).
       01 I PIC 9(8) VALUE 0.
       01 J PIC 9(8) VALUE 0.
       01 Hash PIC 9(8) VALUE 0.
       01 Result1 PIC 9(8) VALUE 0.
       01 Result2 PIC 9(8) VALUE 0.
       01 Res-Output PIC Z(8).
       01 Map-Hash PIC 9(3).
       01 Map-Op PIC X(1).
       01 Map-Key PIC X(64).
       01 Map-Val PIC 9(8).
       01 Val-Idx PIC 9(2).
       01 Val-Len PIC 9(2).
       01 Entry-Updated PIC 9(1).
       01 Entry-Deleted PIC 9(1).
       01 Map.
          03 Bucket OCCURS 256 TIMES.
             05 Bucket-Size PIC 9(3) VALUE 0.
             05 Bucket-Entry OCCURS 256 TIMES.
                10 Entry-Key PIC X(64).
                10 Entry-Val PIC 9(8).

       PROCEDURE DIVISION.
          PERFORM WITH TEST AFTER UNTIL Seq(1:1) = ' '
            ACCEPT Seq
            IF Seq(1:1) <> ' ' THEN
               MOVE ' ' TO Map-Op
               PERFORM WITH TEST AFTER VARYING I FROM 1 BY 1
                                       UNTIL Seq(I:1) = ' '
                  IF Seq(I:1) = ' ' THEN
                     MOVE I TO Val-Len
                     SUBTRACT Val-Idx FROM Val-Len
                     MOVE Seq(Val-Idx:Val-Len) TO Map-Val
                  ELSE
                     IF Seq(I:1) = '=' OR Seq(I:1) = '-' THEN
                        MOVE Hash TO Map-Hash
                        SUBTRACT 1 FROM I
                        MOVE Seq(1:I) TO Map-Key
                        ADD 1 TO I
                        MOVE Seq(I:1) TO Map-Op
                        MOVE I TO Val-Idx
                        ADD 1 TO Val-Idx
                     END-IF
                     ADD FUNCTION ORD(Seq(I:1)) TO Hash
                     ADD -1 TO Hash
                     MULTIPLY 17 BY Hash GIVING Hash
                     MOVE FUNCTION MOD(Hash, 256) TO Hash
                  END-IF
               END-PERFORM

               ADD Hash TO Result1
               MOVE 0 TO Hash

               IF Map-Op = "=" THEN
                  PERFORM map-set
               END-IF
               IF Map-Op = "-" THEN
                  PERFORM map-del
               END-IF
            END-IF
          END-PERFORM

          PERFORM VARYING I FROM 1 BY 1 UNTIL I > 256
             PERFORM VARYING J FROM 1 BY 1 UNTIL J > Bucket-Size(I)
                DISPLAY I '*' J '*' Entry-Val(I,J)
                COMPUTE Result2 = Result2 + (I * J * Entry-Val(I,J))
             END-PERFORM
          END-PERFORM

          MOVE Result1 TO Res-Output.
          DISPLAY 'Part 1: ' Res-Output.
          MOVE Result2 TO Res-Output.
          DISPLAY 'Part 2: ' Res-Output.

          STOP RUN.

       map-set.
          ADD 1 TO Map-Hash
          MOVE 0 TO Entry-Updated
          PERFORM VARYING I FROM 1 BY 1 UNTIL I > Bucket-Size(Map-Hash)
            IF Entry-Key(Map-Hash, I) = Map-Key THEN
               MOVE Map-Val TO Entry-Val(Map-Hash, I)
               MOVE 1 TO Entry-Updated
            END-IF
          END-PERFORM
          IF Entry-Updated = 0 THEN
             ADD 1 TO Bucket-Size(Map-Hash)
             MOVE Map-Key TO Entry-Key(Map-Hash, Bucket-Size(Map-Hash))
             MOVE Map-Val TO Entry-Val(Map-Hash, Bucket-Size(Map-Hash))
          END-IF
          SUBTRACT 1 FROM Map-Hash.

       map-del.
          ADD 1 TO Map-Hash
          MOVE 0 TO Entry-Deleted
          PERFORM VARYING I FROM 1 BY 1 UNTIL I > Bucket-Size(Map-Hash)
            IF Entry-Deleted = 1 OR Entry-Key(Map-Hash,I) = Map-Key THEN
               MOVE I TO J
               ADD 1 TO J
               MOVE Entry-Key(Map-Hash, J) TO Entry-Key(Map-Hash, I)
               MOVE Entry-Val(Map-Hash, J) TO Entry-Val(Map-Hash, I)
               MOVE 1 TO Entry-Deleted
            END-IF
          END-PERFORM
          IF Entry-Deleted = 1 THEN
             SUBTRACT 1 FROM Bucket-Size(Map-Hash)
          END-IF
          SUBTRACT 1 FROM Map-Hash.
