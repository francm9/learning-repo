import 'package:flutter/material.dart';
import 'home_screen.dart';
import 'second_screen.dart';

void main() 
{
  runApp(const MyApp());
}

class MyApp extends StatelessWidget 
{
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp
    (
      theme: ThemeData
      (
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.lightBlue),
        useMaterial3: true,
      ),
      home: const MyHomePage(),
      routes: 
      {
        '/second': (context) => const SecondScreen(),
      },
    );
  }
}

