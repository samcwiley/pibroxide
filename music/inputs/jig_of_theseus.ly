\version "2.25.21"

\include "bagpipe.ly" 

\include "../../../../include/scw_bagpipe.ly"
\include "../../../../include/score_settings.ly"
\include "../../../../include/gitDefinitions.ily"

filename = "four_poster_bed.ly"
tunesetVersion = "Version 1"
source = "John Recknagel"

#(define-bar-line "|.-b" "|." ".|" "|.") %This is for putting all parts in one score!
#(define-bar-line ":|.-b" ":|." ".|:" ":|.") %This is for putting all parts in one score!


#(allow-volta-hook "||")
#(allow-volta-hook "|")


voltaTwo = \markup  { \hspace #20 \italic \fontsize #+5 { "2" }  }

the_jig = {
  \time 6/8 
  \repeat volta 2 {
    \grg c8 [\grd c8 \gre e8] \grg a8 [\grd a8 \gre a8]|
    \grg c8 [\grd c8 \gre e8] \dble e4 f8 |
    \grg c8 [\grd c8 \gre c8] \grg a8 [\grd a8 \gre a8] |
    \grg b8 [e8 \gra e8] \grg d8 [f8 A8] |
    \break
    \grg c8 [\grd c8 \gre c8] \grg a8 [\grd a8 \gre a8] |
    \grg c8 [\grd c8 \gre c8] \dble e4 f8 |
    \grg b8 [d8 \grG d8] \grg G8 [d8 \grG d8] |
    \grg b8 [c8 d8] \dblc c4 A8
  }
  \break
  \repeat volta 2 {
    \grg c8 [\grd c8 \gre c8] e4 f8 |
    \grg c8 [\grd c8 \gre c8] \dble e4 A8 |
    c8 [\grd c8 \gre c8] \dble e4 f8 |
    \grg b8 [e8 \gra e8] \grg d8 [f8 A8] |
    \break
    \grg c8 [\grd c8 \gre c8] e4 f8 |
    \grg c8 [\grd c8 \gre c8] \dble e4 A8 |
    b8 [d8 \grG d8] \grg G8 [d8 \grG d8] |
    \grg b8 [c8 d8] \dblc c4 A8
  }
  \break
  \repeat volta 2 {
    \grg c8 [\grd c8 \gre c8] \grg a4 \taor a8 |
    \dble e4 a8 \dblc c4 \gre a8 |
    \grg c8 [\grd c8 \gre c8] \grg a8 [\grd a8 \gre a8] |
    \grg b8 [e8 \gra e8] \grg d8 [f8 A8] |
    \break
    \grg c8 [\grd c8 \gre c8] \grg a4 \taor a8 |
    \dble e4 a8 \dblc c4 \gre a8 |
    \grg b8 [d8 \grG d8] \grg G8 [d8 \grG d8] |
    \grg b8 [c8 d8] \dblc c4 A8
  }
  \break
  \repeat volta 2 {
    \grg c8 [\grd c8 \gre c8] \grg e8 [c8 \grG c8] |
    \dblA A4 c8 \grg e8 [c8 \grG c8] |
    \grg c8 [\grd c8 \gre c8] \grg a8 [\grd a8 \gre a8] |
    \grg b8 [e8 \gra e8] \grg d8 [f8 A8] |
    \break
    \grg c8 [\grd c8 \gre c8] \grg e8 [c8 \grG c8] |
    \dblA A4 c8 \grg e8 [c8 \grG c8] |
    \grg b8 [d8 \grG d8] \grg G8 [d8 \grG d8] |
    \grg b8 [c8 d8] \dblc c4 A8
  }
  
  }
}

\header { 
          title = \markup  \override #'(line-width . 82) 
          { 
            \column {  
              \center-align {
                \line { The Jig of Theseus }
              }
            }
          }
                  
          subtitle = ""
          composer = "Trad. Fiddle Tune, arr. John Recknagel"
          arranger = "Transcribed S. Wiley"
          meter = "" 
         }    




\paper {
	#(set-paper-size "letter" 'portrait)
}



\score {
	\new GrandStaff <<
		\new Staff = "GHB" <<
			\new Voice {
			        \global
				\four_poster_bed
			}
		>>		
	>>
        \layout { \ScoreLayout 
                  \context { 
                             \Score
                             \override SpacingSpanner.base-shortest-duration = #(ly:make-moment 1/2) 
                           }            
                }
          
                \header{
        }
}5