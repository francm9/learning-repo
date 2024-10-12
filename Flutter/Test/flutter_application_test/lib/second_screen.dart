import 'package:flutter/material.dart';
import 'widgets/footer.dart';

class SecondScreen extends StatefulWidget {
  const SecondScreen({super.key});

  @override
  State<SecondScreen> createState() => _SecondScreenState();
}

class _SecondScreenState extends State<SecondScreen> {

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
                Text('Segunda pantalla'),
              ],
            ),
          ),
          Footer
          (
            focusIcon: 1
          ),
        ],
      ),
    );
  }
}
