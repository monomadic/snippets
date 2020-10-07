-- hello world example

module Main exposing (..)

import Html

main =
   Html.text "Hello, World!"


-- div and styles

module Main exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)

astyledMessage =
  div
    [ style "background-color" "red"
    , style "height" "90px"
    , style "width" "100%"
    ]
    [ text "Hello!"
    ]

main : Html msg
main =
  astyledMessage
