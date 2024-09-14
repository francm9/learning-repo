import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'footer.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {

  @override
  Widget build(BuildContext context) 
  {
    return Scaffold
    (
      appBar: AppBar(),
      body: const Column
      (
        children: 
        [
          Expanded
          (
            child: Column
            (
              children: 
              [
                Text('Pantalla principal'),
              ],
            ),
          ),
          Footer
          (
            focusIcon: 0
          ),
        ],
      ),
    );
  }
}
