[
tictactoe.b -- play tic-tac-toe
(c) 2020 Daniel B. Cristofani
http://brainfuck.org/
  
This program plays tic-tac-toe. I've given it the first move. It needs
interactive i/o, e.g. a command-line brainfuck interpreter or a brainfuck
compiler that produces command-line executables. At the '>' prompt, enter
the number of an empty space, followed by a linefeed, to play a move there.
]

+++>--->->>>->>>>->>-->>>>>>>>>>>>>>>>>>+>>++++++++++[ 
  <+++++++++[  
    <<+++[
      -[<<+>>-]>>-[<<+>>>>-<<-]<[>->>[+]-->>+++<<<<<+[>++>>+>>+<<<<<--]]
      +>+[<+>-]>>>>+++++++[<<++++++++>>-]<<+.[-]<<<<<<
    ]++++++++++.[-]>>
  ]>[-->>]<<[    
    >>--[    
      -[
        -[
          -[
            -[
              -[      
                -[>+<+]<<[
                  >>[-]>-[<<------->+++++++++>>+<-]<-.,<-[>-<+]>[->+>[->]<<<]<
                ]>-->+>,[-]
              ]<[         
                +[[<<]-[>>]<+<-]>[-<+]<<[<<]-<[>[+>>]>[>]+>[-]]
                <-[[+>>]<-->>[>]++++>>+>]
              ]>
            ]<[          
              >-[---<+++]+<---[+++>[---->++++]<<<<<<[>]>[<<[-]>>[>]>]]
              >[<+++>[---->++++]<[<]<[>>[>]+++>+>++++++>+>]>>[[>]->>>>]<<]
              <<+++>++>
            ]>
          ]<[         
            -[[>+>+<<-]>[<+>-]+++>>+++++>>]
            <[<<++[-->>[-]++>]>[[-]+>[<<+>>-]>]<]
          ]>
        ]<[        
          -[ 
            -[-<<<<[-]+>[-]>[<+>>>+<<-]+>>>>>]
            <[<<<--[->[-]>[<<+>+>-]<<[>>+<<-]]++[>]]
          ]<[<[>>+>+<<<-]>>>[<<<+>>>-]]+>
        ]>
      ]<[      
        +[[<]<<[<<]<->>+<-[>>]>[>]<-]<[<]<<+<++[[>+<-]++<[<<->>+]<++]<
        <<<<<<<<+>>>>+>>>>+[
          <<<<->+>+>>+[
            <<<<<<<<->+>>+>>->->>+[
              <<<<<->+>>+>+>+[
                <<<<->+>->->+[
                  <<<<<<<->+>+>>->+>>+[
                    <<<<<->+>>->>+[
                      <<<<->+>>+>+]]]]]]]
        +++[[>+<-]<+++]--->>[<+++>[<->-]>]++[[<->-]>>]>[>]
      ]<
    ]
  ]<
]