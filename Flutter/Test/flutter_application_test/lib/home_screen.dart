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
                Row
                (
                  children: 
                  [
                    Padding
                    (
                      padding: EdgeInsets.fromLTRB(15, 0, 0, 0),
                      child: Text
                      (
                        'Bienvenida de nuevo piti', 
                        style: TextStyle
                        (
                          fontSize: 24,
                          fontWeight: FontWeight.bold,
                        ),
                      ),
                    ),
                  ],
                ),
                Divider()
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
