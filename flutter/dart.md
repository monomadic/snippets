# Dart

## Closures
``` dart
var talk = (s) => print(s);
```

## Map
``` dart
Map m = {'1': 1, '2':2 };
var newMap = new Map.fromIterable(m.keys,
	key: (k) => k , value: (v) => m[v] * 5 );

for (var key in shapes.keys) print(key);
for (var value in shapes.values) print(value);

  var prices;
  prices = currencies.where((f) => watchlist.contains(f["symbol"])).toList();

  for (var currency in prices) {
    print(currency["symbol"] + " " + currency["price_btc"]);
  }

```

## JSON
```dart
import 'dart:async';
import 'dart:convert';
import 'package:http/http.dart' as http;

main(List<String> args) async {
  List currencies = await getCurrencies();
  print(currencies);
}

Future<List> getCurrencies() async {
  String apiUrl = 'https://api.coinmarketcap.com/v1/ticker/?limit=50';
  http.Response response = await http.get(apiUrl);
  return jsonDecode(response.body);
}
```

## Variables

```dart
final items = List<String>.generate(3, (i) => "Item ${i + 1}");
```
