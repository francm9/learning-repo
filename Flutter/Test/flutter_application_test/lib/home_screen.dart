import 'package:flutter/material.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {

  @override
 Widget build(BuildContext context) {
    return Scaffold(
      body: Expanded(
        child: Stack(
          clipBehavior: Clip.none,
          children: [
          
            Container(
              height: 250,
              decoration: BoxDecoration(
                gradient: const LinearGradient(
                  colors: [
                    Color(0xFF9D50BB), 
                    Color(0xFF6E48AA), 
                  ],
                  begin: Alignment.topLeft,
                  end: Alignment.bottomRight,
                ),
                borderRadius: BorderRadius.circular(15),
              ),
            ),
      
            Positioned
            (
              top: 210,
              left: 80,
              child: 
              
              Container
              (
                width: 250, 
                height: 70, 
                decoration: BoxDecoration(
                  color: Colors.white,
                  borderRadius: BorderRadius.circular(10), 
                  boxShadow: 
                  [
                    BoxShadow(
                      color: Colors.grey.withOpacity(0.2),
                      spreadRadius: 5,
                      blurRadius: 5,
                    ),
                  ],
                ),
              ),
            ),
            
          ],
        ),
      ),
    );
  }
}
