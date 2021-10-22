import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:control_pad/control_pad.dart';
import 'package:control_pad/models/pad_button_item.dart';

void main() {
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    SystemChrome.setPreferredOrientations([
      DeviceOrientation.landscapeLeft,
      DeviceOrientation.landscapeRight,
    ]);

    return MaterialApp(
      title: 'GamePad',
      home: HomePage(),
    );
  }
}

class HomePage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        padding: const EdgeInsets.all(20),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            PadButtonsView(
              buttons: const [
                PadButtonItem(index: 0, buttonIcon: Icon(Icons.arrow_right)),
                PadButtonItem(
                    index: 1, buttonIcon: Icon(Icons.arrow_drop_down)),
                PadButtonItem(index: 2, buttonIcon: Icon(Icons.arrow_left)),
                PadButtonItem(index: 3, buttonIcon: Icon(Icons.arrow_drop_up)),
              ],
            ),
            PadButtonsView(buttons: const [
              PadButtonItem(index: 0, buttonText: "B"),
              PadButtonItem(
                index: 1,
                buttonText: "A",
                pressedColor: Colors.red,
              ),
              PadButtonItem(
                index: 2,
                buttonText: "X",
                pressedColor: Colors.green,
              ),
              PadButtonItem(
                index: 3,
                buttonText: "Y",
                pressedColor: Colors.yellow,
              ),
            ])
          ],
        ),
      ),
    );
  }
}
