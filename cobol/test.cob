       IDENTIFICATION DIVISION.
       PROGRAM-ID. SAMPLE.

       DATA DIVISION.
       WORKING-STORAGE SECTION.

         77 fact pic 9(15) comp.
         77 n pic 99.
         77 i pic 99.
         77 ist pic XX.
         77 factst pic X(18).

       PROCEDURE DIVISION.
         move 16 to n
         move 0 to i
         move 1 to fact
         add 1 to fact
         add i to n
         display n " != " fact
