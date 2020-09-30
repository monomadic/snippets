-- Press a button to send a GET request for random cat GIFs.
--
-- Read how it works:
--   https://guide.elm-lang.org/effects/json.html
--

import Browser
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode exposing (Decoder, field, string)



-- MAIN


main =
  Browser.element
    { init = init
    , update = update
    , subscriptions = subscriptions
    , view = view
    }



-- MODEL


type Model
  = Failure
  | Loading
  | Success String


init : () -> (Model, Cmd Msg)
init _ =
  (Loading, getPrice)



-- UPDATE


type Msg
  = TryAgain
  | GotPrice (Result Http.Error String)


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
  case msg of
    TryAgain ->
      (Loading, getPrice)

    GotPrice result ->
      case result of
        Ok url ->
          (Success url, Cmd.none)

        Err _ ->
          (Failure, Cmd.none)



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
  Sub.none



-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ h2 [] [ text "Random Cats" ]
    , viewGif model
    ]


viewGif : Model -> Html Msg
viewGif model =
  case model of
    Failure ->
      div []
        [ text "Error fetching price. "
        , button [ onClick TryAgain ] [ text "Try Again!" ]
        ]

    Loading ->
      text "Loading..."

    Success price ->
      div [][text price]



-- HTTP


getPrice : Cmd Msg
getPrice =
  Http.get
    { url = "https://api.cryptonator.com/api/ticker/btc-usd"
    , expect = Http.expectJson GotPrice responseDecoder
    }


responseDecoder : Decoder String
responseDecoder =
  field "ticker" (field "price" string)