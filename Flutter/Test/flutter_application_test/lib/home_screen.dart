import 'package:flutter/material.dart';
import 'package:flutter_application_test/utils/appcolors.dart';

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
      
            Positioned
            (
              top: 284,
              left: 72,
              child: Container
              (
                width: 249, 
                height: 63, 
                decoration: BoxDecoration(
                  color: AppColors.pink50,
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

            Positioned
            (
              top: 388,
              left: 27,
              child: Container
              (
                width: 344, 
                height: 170, 
                decoration: BoxDecoration
                (
                  color: AppColors.pink50,
                  borderRadius: BorderRadius.circular(10), 
                  boxShadow: 
                  [
                    BoxShadow
                    (
                      color: Colors.grey.withOpacity(0.2),
                      spreadRadius: 5,
                      blurRadius: 5,
                    ),
                  ],
                ),
              ),
            ),

            Positioned
            (
              top: 593,
              left: 27,
              child: Container
              (
                width: 149, 
                height: 208, 
                decoration: BoxDecoration(
                  color: AppColors.pink50,
                  borderRadius: BorderRadius.circular(10), 
                  boxShadow: 
                  [
                    BoxShadow
                    (
                      color: Colors.grey.withOpacity(0.2),
                      spreadRadius: 5,
                      blurRadius: 5,
                    ),
                  ],
                ),
              ),
            ),

            Positioned
            (
              top: 593,
              left: 212,
              child: Container
              (
                width: 159, 
                height: 208, 
                decoration: BoxDecoration(
                  color: AppColors.pink50,
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
