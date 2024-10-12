import 'package:flutter/material.dart';

import 'package:flutter_application_test/widgets/custom_styled_container.dart';

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key});

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {

  @override
 Widget build(BuildContext context) 
 {
    return Scaffold(
      body: Expanded
      (
        child: Stack
        (
          clipBehavior: Clip.none,
          children: 
          [
            Container(
              height: 315,
              decoration: BoxDecoration
              (
                gradient: const LinearGradient
                (
                  colors: 
                  [
                    Color(0xFF9D50BB), 
                    Color(0xFF6E48AA), 
                  ],
                  begin: Alignment.topLeft,
                  end: Alignment.bottomRight,
                ),
                borderRadius: BorderRadius.circular(15),
              ),
            ),

            const CustomStyledContainer
            (
              height: 63, 
              width: 249, 
              top: 284, 
              left: 72, 
              child: Text('texto')
            ),

            const CustomStyledContainer
            (
              height: 170, 
              width: 344, 
              top: 388, 
              left: 27, 
              child: Text('texto')
            ),

            const CustomStyledContainer
            (
              height: 208, 
              width: 159, 
              top: 593, 
              left: 27, 
              child: Text('texto')
            ),

            const CustomStyledContainer
            (
              height: 208, 
              width: 159, 
              top: 593, 
              left: 212, 
              child: Text('texto')
            )
            
          ],
        ),
      ),
    );
  }
}
