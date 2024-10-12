import 'package:flutter/material.dart';
import '../utils/appcolors.dart';

class CustomStyledContainer extends StatelessWidget {
  const CustomStyledContainer({super.key, required this.height, required this.width, required this.top, required this.left, required this.child});

  final double height;
  final double width;
  final double top;
  final double left;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Positioned
    (
      top: top,
      left: left,
      child: Container
      (
        width: width, 
        height: height, 
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
        child: child,
      ),
    );
  }
}